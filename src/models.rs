use diesel::prelude::*;

use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = crate::schema::bookmarks)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Bookmarks {
    pub id: i32,
    pub url: String,
    pub description: String,
}

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = crate::schema::tags)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Tags {
    pub id: i32,
    pub tag: String,
}

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::bookmarks_tags)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct BookmarkTags {
    pub id: i32,
    pub bookmark_id: i32,
    pub tag_id: i32,
}
