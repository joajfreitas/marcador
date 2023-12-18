use std::fs;

use actix_web::{get, web, App, HttpServer, Responder};

use marcador2::Bookmarks;

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
