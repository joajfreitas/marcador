use std::fs;

use actix_web::{get, web, App, HttpServer, Responder};

use marcador2::models::*;
use diesel::prelude::*;

use marcador2::establish_connection;


#[get("/bookmarks")]
async fn endpoint_bookmarks() -> web::Json<Vec<Bookmarks>> {
    use marcador2::schema::bookmarks::dsl::*;

    let connection = &mut establish_connection();
    let results = bookmarks.load(connection).unwrap();
    web::Json(results)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(endpoint_bookmarks)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
