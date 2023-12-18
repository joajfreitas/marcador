use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Bookmarks {
    bookmarks: Vec<Bookmark>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Bookmark {
    url: String,
    description: String,
    tags: Vec<Tag>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tag {
    tag: String
}

impl Bookmarks {
    pub fn from_str(contents: &str) -> Bookmarks {
        serde_json::from_str(&contents).unwrap()
    }

    pub fn bookmarks(&self) -> &Vec<Bookmark> {
        &self.bookmarks
    }

}

impl Bookmark {
    pub fn url(&self) -> &str {
        &self.url
    }
}
