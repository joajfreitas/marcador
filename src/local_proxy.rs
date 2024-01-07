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
use diesel::{delete, insert_into};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use dotenvy::dotenv;

use itertools::intersperse;

use std::{
    env::{temp_dir, var},
    fs::File,
    io::{Read, Write},
    process::Command,
};

use crate::bookmark::Bookmark;
use crate::bookmark_proxy::BookmarkProxy;
use crate::models::{Bookmarks, Tags, BookmarkTags};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub fn establish_connection(url: &str) -> Result<SqliteConnection, String> {
    dotenv().ok();

    SqliteConnection::establish(url).map_err(|_| format!("Error connecting to {}", url))
}

pub struct LocalProxy {
    path: String,
}

impl LocalProxy {
    pub fn new(path: &str) -> LocalProxy {
        let mut connection = establish_connection(path).unwrap();
        connection.run_pending_migrations(MIGRATIONS).unwrap();

        LocalProxy {
            path: path.to_string(),
        }
    }

    fn get_tags(&self, bookmark: &Bookmarks) -> Result<Vec<Tags>, String> {
        use crate::schema::bookmarks_tags::dsl as btdsl;
        use crate::schema::tags::dsl as tdsl;

        let conn = &mut establish_connection(&self.path)?;
        let tag_ids: Vec<i32> = btdsl::bookmarks_tags
            .filter(btdsl::bookmark_id.eq(bookmark.id))
            .select(btdsl::tag_id)
            .get_results(conn)
            .map_err(|err| format!("{:?}", err))?;

        Ok(tag_ids
            .iter()
            .map(|id| {
                tdsl::tags
                    .filter(tdsl::id.eq(id))
                    .select(Tags::as_select())
                    .get_result(conn)
                    .unwrap()
            })
            .collect())
    }

    fn insert_tags(&self, id: i32, tags: &Vec<String>) -> Result<(), String> {
        use crate::schema::bookmarks_tags::dsl as btdsl;
        use crate::schema::tags::dsl as tdsl;

        let conn = &mut establish_connection(&self.path)?;

        for t in tags {
            let ts: Vec<Tags> = tdsl::tags
                .filter(tdsl::tag.eq(t))
                .select(Tags::as_select())
                .get_results(conn)
                .map_err(|err| format!("{:?}", err))?;
            if ts.is_empty() {
                insert_into(tdsl::tags)
                    .values(tdsl::tag.eq(t))
                    .execute(conn)
                    .map_err(|_| "Failed to add tag".to_string())?;
            }

            let tag = tdsl::tags
                .filter(tdsl::tag.eq(t))
                .select(Tags::as_select())
                .get_result(conn)
                .map_err(|err| format!("{:?}", err))?;

            let bts: Vec<BookmarkTags> = btdsl::bookmarks_tags
                .filter(btdsl::tag_id.eq(tag.id))
                .filter(btdsl::bookmark_id.eq(id))
                .select(BookmarkTags::as_select())
                .get_results(conn)
                .map_err(|err| format!("{:?}", err))?;

            if bts.is_empty() {
                insert_into(btdsl::bookmarks_tags)
                    .values((btdsl::bookmark_id.eq(id), btdsl::tag_id.eq(tag.id)))
                    .execute(conn)
                    .map_err(|_| "Failed to add bookmark".to_string())?;
            }
        }

        Ok(())
    }
}

impl BookmarkProxy for LocalProxy {
    fn bookmark(&self, id: i32) -> Result<Bookmark, String> {
        use crate::schema::bookmarks::dsl as bdsl;

        let conn = &mut establish_connection(&self.path)?;
        let bookmark = bdsl::bookmarks
            .filter(bdsl::id.eq(id))
            .select(Bookmarks::as_select())
            .get_result(conn)
            .map_err(|_| "Failed to load bookmark".to_string())?;

        Ok(Bookmark::new(&bookmark, &self.get_tags(&bookmark).unwrap()))
    }

    fn bookmarks(&self) -> Result<Vec<Bookmark>, String> {
        use crate::schema::bookmarks::dsl::*;

        let conn = &mut establish_connection(&self.path)?;
        let bs = bookmarks
            .load(conn)
            .map_err(|_| "Failed to load bookmarks".to_string())?;

        Ok(bs
            .iter()
            .map(|bookmark: &Bookmarks| Bookmark::new(bookmark, &self.get_tags(bookmark).unwrap()))
            .collect())
    }

    fn add(&self, url: &str, description: &str, tags: Vec<String>) -> Result<(), String> {
        use crate::schema::bookmarks::dsl as bdsl;

        let conn = &mut establish_connection(&self.path)?;

        let bs: Vec<Bookmarks> = bdsl::bookmarks
            .filter(bdsl::url.eq(url))
            .select(Bookmarks::as_select())
            .get_results(conn)
            .map_err(|err| format!("bs: {:?}", err))?;

        if !bs.is_empty() {
            return Err("Bookmark already exists".to_string());
        }

        insert_into(bdsl::bookmarks)
            .values((bdsl::url.eq(url), bdsl::description.eq(description)))
            .execute(conn)
            .map_err(|_| "Failed to add bookmark".to_string())?;

        let bookmark_id: i32 = bdsl::bookmarks
            .filter(bdsl::url.eq(url))
            .select(bdsl::id)
            .get_result(conn)
            .map_err(|err| format!("bookmark_id: {:?}", err))?;

        self.insert_tags(bookmark_id, &tags).unwrap();

        Ok(())
    }

    fn delete(&self, identifier: i32) -> Result<(), String> {
        use crate::schema::bookmarks::dsl::*;

        let connection = &mut establish_connection(&self.path)?;
        delete(bookmarks.filter(id.eq(identifier)))
            .execute(connection)
            .map_err(|_| "Failed to delete bookmark".to_string())?;

        Ok(())
    }

    fn update_description(&self, id: i32, description: &str) -> Result<(), String> {
        use crate::schema::bookmarks::dsl as bdsl;

        let conn = &mut establish_connection(&self.path)?;
        diesel::update(bdsl::bookmarks)
            .filter(bdsl::id.eq(id))
            .set(bdsl::description.eq(description))
            .execute(conn)
            .map_err(|_| "Failed to update description".to_string())?;

        Ok(())
    }

    fn update_url(&self, id: i32, url: &str) -> Result<(), String> {
        use crate::schema::bookmarks::dsl as bdsl;

        let conn = &mut establish_connection(&self.path)?;
        diesel::update(bdsl::bookmarks)
            .filter(bdsl::id.eq(id))
            .set(bdsl::url.eq(url))
            .execute(conn)
            .map_err(|_| "Failed to update description".to_string())?;

        Ok(())
    }

    fn update_tags(&self, id: i32, tags: &[String]) -> Result<(), String> {
        use crate::schema::bookmarks_tags::dsl as btdsl;

        let conn = &mut establish_connection(&self.path)?;

        diesel::delete(btdsl::bookmarks_tags)
            .filter(btdsl::bookmark_id.eq(id))
            .execute(conn)
            .map_err(|err| format!("{}", err))?;

        self.insert_tags(id, &tags.to_vec())?;
        Ok(())
    }
}

pub fn edit_bookmark(proxy: &dyn BookmarkProxy, id: i32) {
    let editor = var("EDITOR").unwrap();
    let mut file_path = temp_dir();
    file_path.push("editable");
    let mut file = File::create(&file_path).expect("Could not create file");

    let bookmark = proxy.bookmark(id).unwrap();

    let editable_content = format!(
        "# Description:\n{}\n\n# Url:\n{}\n\n# Tags:\n{}",
        bookmark.bookmark.description,
        bookmark.bookmark.url,
        intersperse(
            bookmark.tags.iter().map(|tag| tag.tag.to_string()),
            ",".to_string()
        )
        .collect::<String>()
    );

    file.write_all(editable_content.as_bytes()).unwrap();

    Command::new(editor)
        .arg(&file_path)
        .status()
        .expect("Something went wrong");

    let mut editable = String::new();

    File::open(file_path)
        .expect("Could not open file")
        .read_to_string(&mut editable)
        .unwrap();

    let lines = editable.lines().collect::<Vec<&str>>();

    let description = lines[1];
    let url = lines[4];

    proxy.update_url(id, url).unwrap();
    proxy.update_description(id, description).unwrap();
    if lines.len() == 8 {
        let tags = dbg!(lines[7]);
        proxy
            .update_tags(
                id,
                &tags
                    .split(',')
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>(),
            )
            .unwrap();
    }
}
