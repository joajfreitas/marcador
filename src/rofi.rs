// Copyright 2020 Tibor Schneider
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//#![deny(missing_docs, missing_debug_implementations, rust_2018_idioms)]

pub mod pango;

use std::io::{Read, Write};
use std::process::{Child, Command, Stdio};
use thiserror::Error;

/// # Rofi Window Builder
/// Rofi struct for displaying user interfaces. This struct is build after the
/// non-consuming builder pattern. You can prepare a window, and draw it
/// multiple times without reconstruction and reallocation. You can choose to
/// return a handle to the child process `RofiChild`, which allows you to kill
/// the process.
#[derive(Debug, Clone)]
pub struct Rofi<'a, T>
where
    T: AsRef<str>,
{
    elements: &'a [T],
    case_sensitive: bool,
    lines: Option<usize>,
    message: Option<String>,
    width: Width,
    format: Format,
    args: Vec<String>,
    sort: bool,
}

/// Rofi child process.
#[derive(Debug)]
pub struct RofiChild<T> {
    num_elements: T,
    p: Child,
}

impl<T> RofiChild<T> {
    fn new(p: Child, arg: T) -> Self {
        Self {
            num_elements: arg,
            p,
        }
    }
    /// Kill the Rofi process
    pub fn kill(&mut self) -> Result<(), Error> {
        Ok(self.p.kill()?)
    }
}

impl RofiChild<String> {
    /// Wait for the result and return the output as a String.
    fn wait_with_output(&mut self) -> Result<(i32, Option<String>), Error> {
        let status = self.p.wait()?;
        let code = status.code().unwrap();
        if status.success() || (10..=30).contains(&code) {
            let mut buffer = String::new();
            if let Some(mut reader) = self.p.stdout.take() {
                reader.read_to_string(&mut buffer)?;
            }
            if buffer.ends_with('\n') {
                buffer.pop();
            }
            if buffer.is_empty() {
                Err(Error::Blank {})
            } else {
                Ok((code, Some(buffer)))
            }
        } else {
            Err(Error::Interrupted {})
        }
    }
}

impl RofiChild<usize> {
    /// Wait for the result and return the output as an usize.
    fn wait_with_output(&mut self) -> Result<(i32, Option<usize>), Error> {
        let status = self.p.wait()?;
        let code = status.code().unwrap();
        if status.success() || (10..=30).contains(&code) {
            let mut buffer = String::new();
            if let Some(mut reader) = self.p.stdout.take() {
                reader.read_to_string(&mut buffer)?;
            }
            if buffer.ends_with('\n') {
                buffer.pop();
            }
            if buffer.is_empty() {
                Err(Error::Blank {})
            } else {
                let idx: isize = buffer.parse::<isize>()?;
                if idx < 0 || idx > self.num_elements as isize {
                    Ok((code, None))
                } else {
                    Ok((code, Some(idx as usize)))
                }
            }
        } else {
            Err(Error::Interrupted {})
        }
    }
}

impl<'a, T> Rofi<'a, T>
where
    T: AsRef<str>,
{
    /// Generate a new, unconfigured Rofi window based on the elements provided.
    pub fn new(elements: &'a [T]) -> Self {
        Self {
            elements,
            case_sensitive: false,
            lines: None,
            width: Width::None,
            format: Format::Text,
            args: Vec::new(),
            sort: false,
            message: None,
        }
    }

    /// Show the window, and return the selected string, including pango
    /// formatting if available
    pub fn run(&self) -> Result<(i32, Option<String>), Error> {
        self.spawn()?.wait_with_output()
    }

    /// show the window, and return the index of the selected string This
    /// function will overwrite any subsequent calls to `self.format`.
    pub fn run_index(&mut self) -> Result<(i32, Option<usize>), Error> {
        self.spawn_index()?.wait_with_output()
    }

    /// Set sort flag
    pub fn set_sort(&mut self) -> &mut Self {
        self.sort = true;
        self
    }

    /// enable pango markup
    pub fn pango(&mut self) -> &mut Self {
        self.args.push("-markup-rows".to_string());
        self
    }

    /// enable password mode
    pub fn password(&mut self) -> &mut Self {
        self.args.push("-password".to_string());
        self
    }

    /// enable message dialog mode (-e)
    pub fn message_only(&mut self, message: impl Into<String>) -> Result<&mut Self, Error> {
        if !self.elements.is_empty() {
            return Err(Error::ConfigErrorMessageAndOptions);
        }
        self.message = Some(message.into());
        Ok(self)
    }

    /// Sets the number of lines.
    /// If this function is not called, use the number of lines provided in the
    /// elements vector.
    pub fn lines(&mut self, l: usize) -> &mut Self {
        self.lines = Some(l);
        self
    }

    /// Set the width of the window (overwrite the theme settings)
    pub fn width(&mut self, w: Width) -> Result<&mut Self, Error> {
        w.check()?;
        self.width = w;
        Ok(self)
    }

    /// Sets the case sensitivity (disabled by default)
    pub fn case_sensitive(&mut self, sensitivity: bool) -> &mut Self {
        self.case_sensitive = sensitivity;
        self
    }

    /// Set the prompt of the rofi window
    pub fn prompt(&mut self, prompt: impl Into<String>) -> &mut Self {
        self.args.push("-p".to_string());
        self.args.push(prompt.into());
        self
    }

    ///
    pub fn message(&mut self, message: impl Into<String>) -> &mut Self {
        self.args.push("-mesg".to_string());
        self.args.push(message.into());
        self
    }

    /// Set the rofi theme
    /// This will make sure that rofi uses `~/.config/rofi/{theme}.rasi`
    pub fn theme(&mut self, theme: Option<impl Into<String>>) -> &mut Self {
        if let Some(t) = theme {
            self.args.push("-theme".to_string());
            self.args.push(t.into());
        }
        self
    }

    /// Set the return format of the rofi call. Default is `Format::Text`. If
    /// you call `self.spawn_index` later, the format will be overwritten with
    /// `Format::Index`.
    pub fn return_format(&mut self, format: Format) -> &mut Self {
        self.format = format;
        self
    }

    ///
    pub fn kb_custom(&mut self, id: u32, shortcut: &str) -> &mut Self {
        self.args.push(format!("-kb-custom-{}", id));
        self.args.push(shortcut.to_string());
        self
    }

    /// Returns a child process with the pre-prepared rofi window
    /// The child will produce the exact output as provided in the elements vector.
    pub fn spawn(&self) -> Result<RofiChild<String>, std::io::Error> {
        Ok(RofiChild::new(self.spawn_child()?, String::new()))
    }

    /// Returns a child process with the pre-prepared rofi window.
    /// The child will produce the index of the chosen element in the vector.
    /// This function will overwrite any subsequent calls to `self.format`.
    pub fn spawn_index(&mut self) -> Result<RofiChild<usize>, std::io::Error> {
        self.format = Format::Index;
        Ok(RofiChild::new(self.spawn_child()?, self.elements.len()))
    }

    fn spawn_child(&self) -> Result<Child, std::io::Error> {
        let mut child = Command::new("rofi")
            .args(match &self.message {
                Some(msg) => vec!["-e", msg],
                None => vec!["-dmenu"],
            })
            .args(&self.args)
            .arg("-format")
            .arg(self.format.as_arg())
            .arg("-l")
            .arg(match self.lines.as_ref() {
                Some(s) => format!("{}", s),
                None => format!("{}", self.elements.len()),
            })
            .arg(match self.case_sensitive {
                true => "-case-sensitive",
                false => "-i",
            })
            .args(match self.width {
                Width::None => vec![],
                Width::Percentage(x) => vec![
                    "-theme-str".to_string(),
                    format!("window {{width: {}%;}}", x),
                ],
                Width::Pixels(x) => vec![
                    "-theme-str".to_string(),
                    format!("window {{width: {}px;}}", x),
                ],
            })
            .arg(match self.sort {
                true => "-sort",
                false => "",
            })
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        if let Some(mut writer) = child.stdin.take() {
            for element in self.elements {
                writer.write_all(element.as_ref().as_bytes())?;
                writer.write_all(b"\n")?;
            }
        }

        Ok(child)
    }
}

static EMPTY_OPTIONS: Vec<String> = vec![];

impl<'a> Rofi<'a, String> {
    /// Generate a new, Rofi window in "message only" mode with the given message.
    pub fn new_message(message: impl Into<String>) -> Self {
        let mut rofi = Self::new(&EMPTY_OPTIONS);
        rofi.message_only(message)
            .expect("Invariant: provided empty options so it is safe to unwrap message_only");
        rofi
    }
}

/// Width of the rofi window to overwrite the default width from the rogi theme.
#[derive(Debug, Clone, Copy)]
pub enum Width {
    /// No width specified, use the default one from the theme
    None,
    /// Width in percentage of the screen, must be between 0 and 100
    Percentage(usize),
    /// Width in pixels, must be greater than 100
    Pixels(usize),
}

impl Width {
    fn check(&self) -> Result<(), Error> {
        match self {
            Self::Percentage(x) => {
                if *x > 100 {
                    Err(Error::InvalidWidth("Percentage must be between 0 and 100"))
                } else {
                    Ok(())
                }
            }
            Self::Pixels(x) => {
                if *x <= 100 {
                    Err(Error::InvalidWidth("Pixels must be larger than 100"))
                } else {
                    Ok(())
                }
            }
            _ => Ok(()),
        }
    }
}

/// Different modes, how rofi should return the results
#[derive(Debug, Clone, Copy)]
pub enum Format {
    /// Regular text, including markup
    #[allow(dead_code)]
    Text,
    /// Text, where the markup is removed
    StrippedText,
    /// Text with the exact user input
    UserInput,
    /// Index of the chosen element
    Index,
}

impl Format {
    fn as_arg(&self) -> &'static str {
        match self {
            Format::Text => "s",
            Format::StrippedText => "p",
            Format::UserInput => "f",
            Format::Index => "i",
        }
    }
}

/// Rofi Error Type
#[derive(Error, Debug)]
pub enum Error {
    /// IO Error
    #[error("IO Error: {0}")]
    IoError(#[from] std::io::Error),
    /// Parse Int Error, only occurs when getting the index.
    #[error("Parse Int Error: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),
    /// Error returned when the user has interrupted the action
    #[error("User interrupted the action")]
    Interrupted,
    /// Error returned when the user chose a blank option
    #[error("User chose a blank line")]
    Blank,
    /// Error returned the width is invalid, only returned in Rofi::width()
    #[error("Invalid width: {0}")]
    InvalidWidth(&'static str),
    /// Error, when the input of the user is not found. This only occurs when
    /// getting the index.
    #[error("User input was not found")]
    NotFound,
    /// Incompatible configuration: cannot specify non-empty options and message_only.
    #[error("Can't specify non-empty options and message_only")]
    ConfigErrorMessageAndOptions,
}
