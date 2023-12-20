use actix_web::{web, App, HttpServer, Result};

use marcador2::models::*;

use marcador2::{AddParams, DeleteParams};
use marcador2::{BookmarkProxy, LocalProxy};

struct State {
    local_proxy: LocalProxy,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    async fn endpoint_list(state: web::Data<State>) -> web::Json<Vec<Bookmarks>> {
        web::Json(state.local_proxy.bookmarks())
    }

    async fn endpoint_add(state: web::Data<State>, info: web::Json<AddParams>) -> Result<String> {
        state.local_proxy.add(&info.url, &info.description, vec![]);
        Ok("nice".to_string())
    }

    async fn endpoint_delete(
        state: web::Data<State>,
        info: web::Json<DeleteParams>,
    ) -> Result<String> {
        state.local_proxy.delete(info.id);

        Ok("nice".to_string())
    }
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(State {
                local_proxy: LocalProxy {},
            }))
            .route("/list", web::get().to(endpoint_list))
            .route("/add", web::post().to(endpoint_add))
            .route("/delete", web::post().to(endpoint_delete))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
