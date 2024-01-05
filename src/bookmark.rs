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
