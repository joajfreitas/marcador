use actix_web::{web, App, HttpServer, Result};

use crate::models::*;

use crate::{AddParams, DeleteParams};
use crate::{BookmarkProxy, LocalProxy};

struct State {
    local_proxy: LocalProxy,
}

async fn endpoint_list(state: web::Data<State>) -> web::Json<Vec<(Bookmarks, Vec<Tags>)>> {
    web::Json(state.local_proxy.bookmarks().unwrap())
}

async fn endpoint_add(state: web::Data<State>, info: web::Json<AddParams>) -> Result<String> {
    state
        .local_proxy
        .add(&info.url, &info.description, vec![])
        .unwrap();
    Ok("nice".to_string())
}

async fn endpoint_delete(
    state: web::Data<State>,
    info: web::Json<DeleteParams>,
) -> Result<web::Json<i32>> {
    state.local_proxy.delete(info.id).unwrap();
    Ok(web::Json(0))
}

pub fn server(db: String) -> Result<(), String> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async_server(db))
        .map_err(|err| format!("{:?}", err))
}

async fn async_server(db: String) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(State {
                local_proxy: LocalProxy::new(&db),
            }))
            .route("/marcador/list", web::get().to(endpoint_list))
            .route("/marcador/add", web::post().to(endpoint_add))
            .route("/marcador/delete", web::post().to(endpoint_delete))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
