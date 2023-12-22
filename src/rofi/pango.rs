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

//! Pango markup language support
//! https://developer.gnome.org/pygtk/stable/pango-markup-language.html

use std::collections::HashMap;
use std::fmt;

/// Structure for writing Pango markup spans
#[derive(Debug, Clone)]
pub struct Pango<'a> {
    content: &'a str,
    options: HashMap<&'static str, &'a str>,
}

impl<'a> Pango<'a> {
    /// Generate a new pango class
    ///
    /// # Usage
    ///
    /// ```
    /// use rofi::pango;
    /// let t = pango::Pango::new("test").build();
    /// assert_eq!(t, "test");
    /// ```
    pub fn new(content: &'a str) -> Pango<'_> {
        Pango {
            content,
            options: HashMap::new(),
        }
    }

    /// Generate a new pango class with options capacity
    ///
    /// # Usage
    ///
    /// ```
    /// use rofi::pango;
    /// let t = pango::Pango::with_capacity("test", 0).build();
    /// assert_eq!(t, "test");
    /// ```
    pub fn with_capacity(content: &'a str, size: usize) -> Pango<'_> {
        Pango {
            content,
            options: HashMap::with_capacity(size),
        }
    }

    /// Generate the pango string
    ///
    /// # Usage
    ///
    /// ```
    /// use rofi::pango;
    /// let t = pango::Pango::new("test")
    ///     .slant_style(pango::SlantStyle::Italic)
    ///     .size(pango::FontSize::Small)
    ///     .build();
    /// assert!(t == "<span style='italic' size='small'>test</span>" ||
    ///         t == "<span size='small' style='italic'>test</span>");
    /// ```
    pub fn build(&mut self) -> String {
        self.to_string()
    }

    /// Generates a pango string based on the options, but with a different
    /// content.
    ///
    /// ```
    /// use rofi::pango;
    /// let mut p = pango::Pango::new("");
    /// p.slant_style(pango::SlantStyle::Italic);
    /// p.size(pango::FontSize::Small);
    /// let t = p.build_content("test");
    /// assert!(t == "<span style='italic' size='small'>test</span>" ||
    ///         t == "<span size='small' style='italic'>test</span>");
    /// ```
    pub fn build_content(&self, content: &str) -> String {
        self.to_string_with_content(content)
    }

    /// Set the font
    ///
    /// # Usage
    ///
    /// ```
    /// use rofi::pango;
    /// let t = pango::Pango::new("test")
    ///     .font_description("Sans Italic 12")
    ///     .build();
    /// assert_eq!(t, "<span font_desc='Sans Italic 12'>test</span>");
    /// ```
    pub fn font_description(&mut self, font: &'a str) -> &mut Self {
        self.options.insert("font_desc", font);
        self
    }

    /// set the font family
    ///
    /// # Usage
    ///
    /// ```
    /// use rofi::pango;
    /// let t = pango::Pango::new("test")
    ///     .font_family(pango::FontFamily::Monospace)
    ///     .build();
    /// assert_eq!(t, "<span face='monospace'>test</span>");
    /// ```
    pub fn font_family(&mut self, family: FontFamily) -> &mut Self {
        self.options.insert(
            "face",
            match family {
                FontFamily::Normal => "normal",
                FontFamily::Sans => "sans",
                FontFamily::Serif => "serif",
                FontFamily::Monospace => "monospace",
            },
        );
        self
    }

    /// Set the size of the font, relative to the configured font size
    ///
    /// # Usage
    ///
    /// ```
    /// use rofi::pango;
    /// let t = pango::Pango::new("test")
    ///     .size(pango::FontSize::Huge)
    ///     .build();
    /// assert_eq!(t, "<span size='x-large'>test</span>");
    /// ```
    pub fn size(&mut self, size: FontSize) -> &mut Self {
        self.options.insert(
            "size",
            match size {
                FontSize::VeryTiny => "xx-small",
                FontSize::Tiny => "x-small",
                FontSize::Small => "small",
                FontSize::Normal => "medium",
                FontSize::Large => "large",
                FontSize::Huge => "x-large",
                FontSize::VeryHuge => "xx-large",
                FontSize::Smaller => "smaller",
                FontSize::Larger => "larger",
            },
        );
        self
    }

    /// Set the slant style (italic / oblique / normal)
    ///
    /// # Usage
    ///
    /// ```
    /// use rofi::pango;
    /// let t = pango::Pango::new("test")
    ///     .slant_style(pango::SlantStyle::Oblique)
    ///     .build();
    /// assert_eq!(t, "<span style='oblique'>test</span>");
    /// ```
    pub fn slant_style(&mut self, style: SlantStyle) -> &mut Self {
        self.options.insert(
            "style",
            match style {
                SlantStyle::Normal => "normal",
                SlantStyle::Oblique => "oblique",
                SlantStyle::Italic => "italic",
            },
        );
        self
    }

    /// Set the font weight
    ///
    /// # Usage
    ///
    /// ```
    /// use rofi::pango;
    /// let t = pango::Pango::new("test")
    ///     .weight(pango::Weight::Bold)
    ///     .build();
    /// assert_eq!(t, "<span weight='bold'>test</span>");
    /// ```
    pub fn weight(&mut self, weight: Weight) -> &mut Self {
        self.options.insert(
            "weight",
            match weight {
                Weight::Thin => "100",
                Weight::UltraLight => "ultralight",
                Weight::Light => "light",
                Weight::Normal => "normal",
                Weight::Medium => "500",
                Weight::SemiBold => "600",
                Weight::Bold => "bold",
                Weight::UltraBold => "ultrabold",
                Weight::Heavy => "heavy",
                Weight::UltraHeavy => "1000",
            },
        );
        self
    }

    /// Set the alpha of the text
    /// Important: alpha must be fo the form: XX%, where XX is a number between 0 and 100.
    ///
    /// # Usage
    ///
    /// ```
    /// use rofi::pango;
    /// let t = pango::Pango::new("test")
    ///     .alpha("50%")
    ///     .build();
    /// assert_eq!(t, "<span alpha='50%'>test</span>");
    /// ```
    pub fn alpha(&mut self, alpha: &'a str) -> &mut Self {
        self.options.insert("alpha", alpha);
        self
    }

    /// Use smallcaps
    ///
    /// # Usage
    ///
    /// ```
    /// use rofi::pango;
    /// let t = pango::Pango::new("test")
    ///     .small_caps()
    ///     .build();
    /// assert_eq!(t, "<span variant='smallcaps'>test</span>");
    /// ```
    pub fn small_caps(&mut self) -> &mut Self {
        self.options.insert("variant", "smallcaps");
        self
    }

    /// Set the stretch (expanded or condensed)
    ///
    /// # Usage
    ///
    /// ```
    /// use rofi::pango;
    /// let t = pango::Pango::new("test")
    ///     .stretch(pango::FontStretch::Condensed)
    ///     .build();
    /// assert_eq!(t, "<span stretch='condensed'>test</span>");
    /// ```
    pub fn stretch(&mut self, stretch: FontStretch) -> &mut Self {
        self.options.insert(
            "stretch",
            match stretch {
                FontStretch::UltraCondensed => "ultracondensed",
                FontStretch::ExtraCondensed => "extracondensed",
                FontStretch::Condensed => "condensed",
                FontStretch::SemiCondensed => "semicondensed",
                FontStretch::Normal => "normal",
                FontStretch::SemiExpanded => "semiexpanded",
                FontStretch::Expanded => "expanded",
                FontStretch::ExtraExpanded => "extraexpanded",
                FontStretch::UltraExpanded => "ultraexpanded",
            },
        );
        self
    }

    /// Set the foreground color
    ///
    /// # Usage
    ///
    /// ```
    /// use rofi::pango;
    /// let t = pango::Pango::new("test")
    ///     .fg_color("#00FF00")
    ///     .build();
    /// assert_eq!(t, "<span foreground='#00FF00'>test</span>");
    /// ```
    pub fn fg_color(&mut self, color: &'a str) -> &mut Self {
        self.options.insert("foreground", color);
        self
    }

    /// Set the background color
    ///
    /// # Usage
    ///
    /// ```
    /// use rofi::pango;
    /// let t = pango::Pango::new("test")
    ///     .bg_color("#00FF00")
    ///     .build();
    /// assert_eq!(t, "<span background='#00FF00'>test</span>");
    /// ```
    pub fn bg_color(&mut self, color: &'a str) -> &mut Self {
        self.options.insert("background", color);
        self
    }

    /// Set the underline style
    ///
    /// # Usage
    ///
    /// ```
    /// use rofi::pango;
    /// let t = pango::Pango::new("test")
    ///     .underline(pango::Underline::Double)
    ///     .build();
    /// assert_eq!(t, "<span underline='double'>test</span>");
    /// ```
    pub fn underline(&mut self, underline: Underline) -> &mut Self {
        self.options.insert(
            "underline",
            match underline {
                Underline::None => "none",
                Underline::Single => "single",
                Underline::Double => "double",
                Underline::Low => "low",
            },
        );
        self
    }

    /// set the font to strike through
    ///
    /// # Usage
    ///
    /// ```
    /// use rofi::pango;
    /// let t = pango::Pango::new("test")
    ///     .strike_through()
    ///     .build();
    /// assert_eq!(t, "<span strikethrough='true'>test</span>");
    /// ```
    pub fn strike_through(&mut self) -> &mut Self {
        self.options.insert("strikethrough", "true");
        self
    }

    fn to_string_with_content(&self, content: &str) -> String {
        if self.options.is_empty() {
            content.to_string()
        } else {
            format!(
                "<span {}>{}</span>",
                self.options
                    .iter()
                    .map(|(k, v)| format!("{}='{}'", k, v))
                    .collect::<Vec<String>>()
                    .join(" "),
                content
            )
        }
    }
}

/// Enumeration over all available font families
#[derive(Debug, Clone, Copy)]
pub enum FontFamily {
    /// Normal font
    Normal,
    /// Sans Serif font
    Sans,
    /// Font including serif
    Serif,
    /// Monospaced font
    Monospace,
}

/// Enumeration over all avaliable font sizes
#[derive(Debug, Clone, Copy)]
pub enum FontSize {
    /// Very tiny font size, corresponsds to xx-small
    VeryTiny,
    /// Tiny font size, corresponds to x-small
    Tiny,
    /// Small font size, corresponds to small
    Small,
    /// Normal font size (default), corresponds to medium
    Normal,
    /// Large font size, corresponds to large
    Large,
    /// Huge font size, corresponds to x-large
    Huge,
    /// Very huge font size, corresponds to xx-large
    VeryHuge,
    /// Relative font size, makes content smaller than the parent
    Smaller,
    /// Relative font size, makes content larger than the parent
    Larger,
}

/// Enumeration over all possible slant styles
#[derive(Debug, Clone, Copy)]
pub enum SlantStyle {
    /// No slant
    Normal,
    /// Oblique, normal font skewed
    Oblique,
    /// Italic font, (different face)
    Italic,
}

/// Enumeration over all possible weights
#[derive(Debug, Clone, Copy)]
pub enum Weight {
    /// Thin weight (=100)
    Thin,
    /// Ultralight weight (=200)
    UltraLight,
    /// Light weight (=300)
    Light,
    /// Normal weight (=400)
    Normal,
    /// Medium weight (=500)
    Medium,
    /// SemiBold weight (=600)
    SemiBold,
    /// Bold weight (=700)
    Bold,
    /// Ultrabold weight (=800)
    UltraBold,
    /// Heavy (=900)
    Heavy,
    /// UltraHeavy weight (=1000)
    UltraHeavy,
}

/// enumeration over all possible font stretch modes
#[derive(Debug, Clone, Copy)]
pub enum FontStretch {
    /// UltraCondensed, letters are extremely close together
    UltraCondensed,
    /// ExtraCondensed, letters are very close together
    ExtraCondensed,
    /// Condensed, letters are close together
    Condensed,
    /// SemiCondensed, letters somewhat are close together
    SemiCondensed,
    /// Normal, normal spacing as defined by the font
    Normal,
    /// SemiExpanded, letters somewhat are far apart
    SemiExpanded,
    /// Expanded, letters somewhat far apart
    Expanded,
    /// ExtraExpanded, letters very far apart
    ExtraExpanded,
    /// UltraExpanded, letters extremely far apart
    UltraExpanded,
}

/// enumeration over all possible underline modes
#[derive(Debug, Clone, Copy)]
pub enum Underline {
    /// No underline mode
    None,
    /// Single, normal underline
    Single,
    /// Double
    Double,
    /// Low, only the lower line of double is drawn
    Low,
}

impl<'a> fmt::Display for Pango<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.options.is_empty() {
            write!(f, "{}", self.content)
        } else {
            write!(f, "<span")?;
            for (k, v) in self.options.iter() {
                write!(f, " {}='{}'", k, v)?;
            }
            write!(f, ">{}</span>", self.content)
        }
    }
}
