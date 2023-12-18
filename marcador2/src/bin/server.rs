use std::fs;

use actix_web::{get, web, App, HttpServer, Responder};

use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

use marcador2::Bookmarks;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> web::Json<Bookmarks> {
    let contents = fs::read_to_string("marcador.json").unwrap();
    web::Json(Bookmarks::from_str(&contents))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(greet)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
