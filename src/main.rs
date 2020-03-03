mod handlers;
mod models;
mod config;
mod db;
mod errors;

use actix_web::{HttpServer, App, web, middleware};
use std::io;
use dotenv::dotenv;
use tokio_postgres::NoTls;
use crate::handlers::*;
use crate::config::Config;
use slog::info;
use crate::models::AppState;


#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let config = Config::from_env().unwrap();

    let pool = config.pg.create_pool(NoTls).unwrap();

    let log = Config::configure_log();

    info!(log, "Starting server at http://{}:{}", config.server.host, config.server.port);

    HttpServer::new(move || {

        App::new()
        .data(AppState { pool: pool.clone(), log: log.clone() })
        .wrap(middleware::Logger::default())
        .route("/", web::get().to(status))
        .route("/todos{_:/?}", web::get().to(todos))
        .route("/todos{_:/?}", web::post().to(create_todo))
        .route("/todos/{list_id}{_:/?}", web::get().to(get_todo))
        .route("/todos/{list_id}/items{_:/?}", web::get().to(items))
        .route("/todos/{list_id}/items{_:/?}", web::post().to(create_item))
        .route("/todos/{list_id}/items/{item_id}{_:/?}", web::get().to(get_item))
        .route("/todos/{list_id}/items/{item_id}{_:/?}", web::put().to(check_todo))
    }
    )
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}

#[cfg(test)]
#[cfg(feature = "integration")]
mod integration_tests;