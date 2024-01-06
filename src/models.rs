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
