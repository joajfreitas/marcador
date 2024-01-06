// Copyright 2024 João Freitas
//
// This program is free software: you can redistribute it and/or modify it under the terms of
// the GNU General Public License as published by the Free Software Foundation, either
// version 3 of the License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT ANY
// WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR
// A PARTICULAR PURPOSE. See the GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with this
// program. If not, see <https://www.gnu.org/licenses/>.

use itertools::intersperse;
use serde::{Deserialize, Serialize};
use std::fmt;

use colored::Colorize;

use crate::models::{Bookmarks, Tags};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Bookmark {
    pub bookmark: Bookmarks,
    pub tags: Vec<Tags>,
}

impl Bookmark {
    pub fn new(bookmark: &Bookmarks, tags: &[Tags]) -> Bookmark {
        Bookmark {
            bookmark: bookmark.clone(),
            tags: tags.to_vec(),
        }
    }
}

impl fmt::Display for Bookmark {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}. {}\n  {} {}",
            self.bookmark.id.to_string().cyan(),
            self.bookmark.description.to_string().green(),
            ">".red(),
            self.bookmark.url.to_string().yellow()
        )?;

        if !self.tags.is_empty() {
            write!(
                f,
                "\n{} {}",
                "#".red(),
                intersperse(
                    self.tags.iter().map(|tag| tag.tag.blue().to_string()),
                    ",".blue().to_string()
                )
                .collect::<String>()
            )
        } else {
            Ok(())
        }
    }
}
