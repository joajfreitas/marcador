pub mod models;
pub mod schema;

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
    fn bookmarks(&self) -> Vec<Bookmarks>;
    fn add(&self, url: &str, description: &str, tags: Vec<String>);
    fn delete(&self, id: i32);
}

pub struct LocalProxy {}

impl BookmarkProxy for LocalProxy {
    fn bookmarks(&self) -> Vec<Bookmarks> {
        use self::schema::bookmarks::dsl::*;
        let conn = &mut establish_connection();
        bookmarks.load(conn).unwrap()
    }

    fn add(&self, link: &str, desc: &str, _tags: Vec<String>) {
        use self::schema::bookmarks::dsl::*;

        let conn = &mut establish_connection();
        insert_into(bookmarks)
            .values((url.eq(link), description.eq(desc)))
            .execute(conn)
            .unwrap();
    }

    fn delete(&self, identifier: i32) {
        use self::schema::bookmarks::dsl::*;

        let connection = &mut establish_connection();
        delete(bookmarks.filter(id.eq(identifier)))
            .execute(connection)
            .unwrap();
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
    fn bookmarks(&self) -> Vec<Bookmarks> {
        reqwest::blocking::get(&self.list_endpoint)
            .unwrap()
            .json::<Vec<Bookmarks>>()
            .unwrap()
    }

    fn add(&self, link: &str, desc: &str, _tags: Vec<String>) {
        let client = reqwest::blocking::Client::new();
        client
            .post(&self.add_endpoint)
            .json(&AddParams {
                url: link.to_string(),
                description: desc.to_string(),
            })
            .send()
            .unwrap();
    }
    fn delete(&self, identifier: i32) {
        let client = reqwest::blocking::Client::new();
        client
            .post(&self.delete_endpoint)
            .json(&DeleteParams { id: identifier })
            .send()
            .unwrap();
    }
}

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
