pub mod models;
pub mod rofi;
pub mod schema;
pub mod server;

use diesel::prelude::*;
use diesel::{delete, insert_into};
use dotenvy::dotenv;
use std::env;

use serde::{Deserialize, Serialize};

use models::Bookmarks;

#[derive(Serialize, Deserialize)]
pub struct AddParams {
    pub url: String,
    pub description: String,
}

#[derive(Serialize, Deserialize)]
pub struct DeleteParams {
    pub id: i32,
}

pub trait BookmarkProxy {
    fn bookmarks(&self) -> Result<Vec<Bookmarks>, String>;
    fn add(&self, url: &str, description: &str, tags: Vec<String>) -> Result<(), String>;
    fn delete(&self, id: i32) -> Result<(), String>;
}

pub struct LocalProxy {}

impl BookmarkProxy for LocalProxy {
    fn bookmarks(&self) -> Result<Vec<Bookmarks>, String> {
        use self::schema::bookmarks::dsl::*;
        let conn = &mut establish_connection()?;
        bookmarks
            .load(conn)
            .map_err(|_| "Failed to load bookmarks".to_string())
    }

    fn add(&self, link: &str, desc: &str, _tags: Vec<String>) -> Result<(), String> {
        use self::schema::bookmarks::dsl::*;

        let conn = &mut establish_connection()?;
        insert_into(bookmarks)
            .values((url.eq(link), description.eq(desc)))
            .execute(conn)
            .map_err(|_| "Failed to add bookmark".to_string())?;

        Ok(())
    }

    fn delete(&self, identifier: i32) -> Result<(), String> {
        use self::schema::bookmarks::dsl::*;

        let connection = &mut establish_connection()?;
        delete(bookmarks.filter(id.eq(identifier)))
            .execute(connection)
            .map_err(|_| "Failed to delete bookmark".to_string())?;

        Ok(())
    }
}

pub struct RemoteProxy {
    list_endpoint: String,
    add_endpoint: String,
    delete_endpoint: String,
}

impl RemoteProxy {
    pub fn new(url: &str) -> Self {
        Self {
            list_endpoint: url.to_string() + "/list",
            add_endpoint: url.to_string() + "/add",
            delete_endpoint: url.to_string() + "/delete",
        }
    }
}

impl BookmarkProxy for RemoteProxy {
    fn bookmarks(&self) -> Result<Vec<Bookmarks>, String> {
        Ok(reqwest::blocking::get(&self.list_endpoint)
            .map_err(|_| "Failed to get web resource")?
            .json::<Vec<Bookmarks>>()
            .map_err(|_| "Failed to parse json")?)
    }

    fn add(&self, link: &str, desc: &str, _tags: Vec<String>) -> Result<(), String> {
        let client = reqwest::blocking::Client::new();
        client
            .post(&self.add_endpoint)
            .json(&AddParams {
                url: link.to_string(),
                description: desc.to_string(),
            })
            .send()
            .map_err(|_| "Failed to send post request")?;

        Ok(())
    }
    fn delete(&self, identifier: i32) -> Result<(), String> {
        let client = reqwest::blocking::Client::new();
        client
            .post(&self.delete_endpoint)
            .json(&DeleteParams { id: identifier })
            .send()
            .map_err(|_| "Failed to send post request")?;

        Ok(())
    }
}

pub fn establish_connection() -> Result<SqliteConnection, String> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").map_err(|_| "DATABASE_URL must be set")?;
    SqliteConnection::establish(&database_url)
        .map_err(|_| format!("Error connecting to {}", database_url))
}
