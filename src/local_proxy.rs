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

use crate::bookmark::Bookmark;
use crate::bookmark_proxy::BookmarkProxy;
use crate::models::{Bookmarks, Tags};

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
}

impl BookmarkProxy for LocalProxy {
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
        use crate::schema::bookmarks_tags::dsl as btdsl;
        use crate::schema::tags::dsl as tdsl;

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

        for t in tags {
            let ts: Vec<Tags> = tdsl::tags
                .filter(tdsl::tag.eq(&t))
                .select(Tags::as_select())
                .get_results(conn)
                .map_err(|err| format!("{:?}", err))?;
            if !ts.is_empty() {
                continue;
            }
            insert_into(tdsl::tags)
                .values(tdsl::tag.eq(&t))
                .execute(conn)
                .map_err(|_| "Failed to add tag".to_string())?;

            let tag = tdsl::tags
                .filter(tdsl::tag.eq(&t))
                .select(Tags::as_select())
                .get_result(conn)
                .map_err(|err| format!("{:?}", err))?;

            insert_into(btdsl::bookmarks_tags)
                .values((btdsl::bookmark_id.eq(bookmark_id), btdsl::tag_id.eq(tag.id)))
                .execute(conn)
                .map_err(|_| "Failed to add bookmark".to_string())?;
        }

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
}
