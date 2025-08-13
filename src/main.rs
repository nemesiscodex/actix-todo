// File: src/main.rs
// High-level: Bootstraps the Actix-Web server, configures shared state, and wires HTTP routes to handlers.
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
    // Load environment variables from .env for local development convenience
    dotenv().ok();

    // Read runtime configuration (server + Postgres) from environment
    let config = Config::from_env().unwrap();

    // Create a Postgres connection pool once and share it across requests
    // Add retry logic for database connection
    let pool = loop {
        match config
            .pg
            .create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)
        {
            Ok(pool) => {
                eprintln!("Database connection pool created successfully");
                break pool;
            }
            Err(e) => {
                eprintln!("Failed to create database pool: {}. Retrying in 5 seconds...", e);
                std::thread::sleep(std::time::Duration::from_secs(5));
            }
        }
    };

    // Configure structured logging early so subsequent logs are formatted
    let log = Config::configure_log();

    info!(
        log,
        "Starting server at http://{}:{}",
        config.server.host,
        config.server.port
    );
    
    // Test database connection before starting server
    match pool.get().await {
        Ok(_) => info!(log, "Database connection test successful"),
        Err(e) => {
            eprintln!("Database connection test failed: {}", e);
            std::process::exit(1);
        }
    }

    // App shared state: injected into handlers via app_data
    let state = web::Data::new(AppState { pool, log });

    HttpServer::new(move || {
        // Build the per-worker App, cloning the shared state handle
        App::new()
            .app_data(state.clone())
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
    // Bind the TCP listener using configured host:port and start the server
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}

#[cfg(test)]
#[cfg(feature = "integration")]
mod integration_tests;
