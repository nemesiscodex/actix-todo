mod config;
mod db;
mod errors;
mod handlers;
mod models;

use crate::config::Config;
use crate::handlers::*;
use crate::models::AppState;
use actix_web::{middleware, web, App, HttpServer};
use dotenv::dotenv;
use slog::info;
use std::io;
use tokio_postgres::NoTls;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let config = Config::from_env().unwrap();

    let pool = config.pg.create_pool(NoTls).unwrap();

    let log = Config::configure_log();

    info!(
        log,
        "Starting server at http://{}:{}", config.server.host, config.server.port
    );

    HttpServer::new(move || {
        App::new()
            .data(AppState {
                pool: pool.clone(),
                log: log.clone(),
            })
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(status))
            .route("/todos{_:/?}", web::get().to(todos))
            .route("/todos{_:/?}", web::post().to(create_todo))
            .route("/todos/{list_id}{_:/?}", web::get().to(get_todo))
            .route("/todos/{list_id}/items{_:/?}", web::get().to(items))
            .route("/todos/{list_id}/items{_:/?}", web::post().to(create_item))
            .route(
                "/todos/{list_id}/items/{item_id}{_:/?}",
                web::get().to(get_item),
            )
            .route(
                "/todos/{list_id}/items/{item_id}{_:/?}",
                web::put().to(check_todo),
            )
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}

#[cfg(test)]
#[cfg(feature = "integration")]
mod integration_tests;
