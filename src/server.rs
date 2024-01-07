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

use actix_web::{web, App, HttpServer, Result};
use clap::Parser;

use serde::{Deserialize, Serialize};

use crate::bookmark::Bookmark;

use crate::config::{Config, ServerConfig};
use crate::{BookmarkProxy, LocalProxy};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(long)]
    pub db: Option<String>,
    #[arg(long)]
    pub host: Option<String>,
    #[arg(long)]
    pub port: Option<u16>,
    #[arg(long)]
    pub root: Option<String>,
}

struct State {
    local_proxy: LocalProxy,
}

#[derive(Serialize, Deserialize)]
pub struct AddParams {
    pub url: String,
    pub description: String,
}

#[derive(Serialize, Deserialize)]
pub struct DeleteParams {
    pub id: i32,
}

async fn endpoint_bookmark(state: web::Data<State>, info: web::Json<i32>) -> web::Json<Bookmark> {
    web::Json(state.local_proxy.bookmark(info.0).unwrap())
}

async fn endpoint_list(state: web::Data<State>) -> web::Json<Vec<Bookmark>> {
    web::Json(state.local_proxy.bookmarks().unwrap())
}

async fn endpoint_add(
    state: web::Data<State>,
    info: web::Json<AddParams>,
) -> Result<web::Json<i32>> {
    state
        .local_proxy
        .add(&info.url, &info.description, vec![])
        .unwrap();
    Ok(web::Json(0))
}

async fn endpoint_delete(
    state: web::Data<State>,
    info: web::Json<DeleteParams>,
) -> Result<web::Json<i32>> {
    state.local_proxy.delete(info.id).unwrap();
    Ok(web::Json(0))
}

async fn endpoint_update_description(
    state: web::Data<State>,
    info: web::Json<(i32, String)>,
) -> Result<web::Json<i32>> {
    state
        .local_proxy
        .update_description(info.0 .0, &info.0 .1)
        .unwrap();
    Ok(web::Json(0))
}

async fn endpoint_update_url(
    state: web::Data<State>,
    info: web::Json<(i32, String)>,
) -> Result<web::Json<i32>> {
    state.local_proxy.update_url(info.0 .0, &info.0 .1).unwrap();
    Ok(web::Json(0))
}

async fn endpoint_update_tags(
    state: web::Data<State>,
    info: web::Json<(i32, Vec<String>)>,
) -> Result<web::Json<i32>> {
    state
        .local_proxy
        .update_tags(info.0 .0, &info.0 .1)
        .unwrap();
    Ok(web::Json(0))
}

pub fn server(cli: Cli) -> Result<(), String> {
    let config = Config::read().ok_or("Failed to read config".to_string())?;

    let mut server_config = if let Some(server_config) = config.server {
        server_config
    } else {
        ServerConfig::default()
    };

    server_config.set_db(&cli.db);
    server_config.set_host(&cli.host);
    server_config.set_port(&cli.port);
    server_config.set_root(&cli.root);

    println!(
        "Running server {}:{}{}",
        server_config.get_host(),
        server_config.get_port(),
        server_config.get_root()
    );
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async_server(
            server_config.db.ok_or("Expected db path")?,
            server_config.host.unwrap_or("127.0.0.1".to_string()),
            server_config.port.unwrap_or(8080),
            server_config.root.unwrap_or("/".to_string()),
        ))
        .map_err(|err| format!("{:?}", err))
}

async fn async_server(db: String, host: String, port: u16, root: String) -> std::io::Result<()> {
    let (
        bookmark_endpoint,
        list_endpoint,
        add_endpoint,
        delete_endpoint,
        update_description_endpoint,
        update_url_endpoint,
        update_tags_endpoint,
    ) = if root == "/" {
        (
            "/bookmark".to_string(),
            "/list".to_string(),
            "/add".to_string(),
            "/delete".to_string(),
            "/update_description".to_string(),
            "/update_url".to_string(),
            "/update_tags".to_string(),
        )
    } else {
        (
            root.clone() + "/bookmark",
            root.clone() + "/list",
            root.clone() + "/add",
            root.clone() + "/delete",
            root.clone() + "/update_description",
            root.clone() + "/update_url",
            root.clone() + "/update_tags",
        )
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(State {
                local_proxy: LocalProxy::new(&db),
            }))
            .route(&bookmark_endpoint, web::get().to(endpoint_bookmark))
            .route(&list_endpoint, web::get().to(endpoint_list))
            .route(&add_endpoint, web::post().to(endpoint_add))
            .route(&delete_endpoint, web::post().to(endpoint_delete))
            .route(
                &update_description_endpoint,
                web::post().to(endpoint_update_description),
            )
            .route(&update_url_endpoint, web::post().to(endpoint_update_url))
            .route(&update_tags_endpoint, web::post().to(endpoint_update_tags))
    })
    .bind((host, port))?
    .run()
    .await
}
