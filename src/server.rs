use actix_web::{web, App, HttpServer, Result};

use crate::bookmark::Bookmark;

use crate::config::ServerConfig;
use crate::{AddParams, DeleteParams};
use crate::{BookmarkProxy, LocalProxy};

struct State {
    local_proxy: LocalProxy,
}

async fn endpoint_list(state: web::Data<State>) -> web::Json<Vec<Bookmark>> {
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

pub fn server(config: ServerConfig) -> Result<(), String> {
    println!(
        "Running server {}:{}{}",
        config.get_host(),
        config.get_port(),
        config.get_root()
    );
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async_server(
            config.db.ok_or("Expected db path")?,
            config.host.unwrap_or("127.0.0.1".to_string()),
            config.port.unwrap_or(8080),
            config.root.unwrap_or("/".to_string()),
        ))
        .map_err(|err| format!("{:?}", err))
}

async fn async_server(db: String, host: String, port: u16, root: String) -> std::io::Result<()> {
    let (list_endpoint, add_endpoint, delete_endpoint) = if root == "/" {
        (
            "/list".to_string(),
            "/add".to_string(),
            "/delete".to_string(),
        )
    } else {
        (
            root.clone() + "/list",
            root.clone() + "/add",
            root + "/delete",
        )
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(State {
                local_proxy: LocalProxy::new(&db),
            }))
            .route(&list_endpoint, web::get().to(endpoint_list))
            .route(&add_endpoint, web::post().to(endpoint_add))
            .route(&delete_endpoint, web::post().to(endpoint_delete))
    })
    .bind((host, port))?
    .run()
    .await
}
