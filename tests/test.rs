extern crate actix_todo;


use actix_web::{test, App, web};
use dotenv::dotenv;
use actix_todo::config::Config;
use actix_todo::models::{AppState, TodoList};
use actix_todo::handlers::todos;
use tokio_postgres::NoTls;
use slog::{info};
use std::env;

fn setup () -> AppState {

    dotenv().ok();

    let log = Config::configure_log();

    info!(log, "Server host: {:?}", env::var("SERVER.HOST").unwrap());

    let config = Config::from_env().unwrap();

    let pool = config.pg.create_pool(NoTls).unwrap();

    AppState {
        pool: pool.clone(),
        log: log.clone()
    }

}

#[actix_rt::test]
async fn test_get_todos() {

    let state = setup();

    let log = state.log.clone();

    let app = App::new()
        .data(state)
        .route("/todos", web::get().to(todos));

    let mut app = test::init_service(app).await;

    let req = test::TestRequest::get().uri("/todos").to_request();

    let todos: Vec<TodoList> = test::read_response_json(&mut app, req).await;

    info!(log, "GET /todos -> {:?}", todos);

}
