extern crate actix_todo;


use actix_web::{test, App, web};
use dotenv::dotenv;
use actix_todo::config::Config;
use actix_todo::models::{AppState, TodoList};
use actix_todo::handlers::todos;
use tokio_postgres::NoTls;
use slog::{info};

fn setup () -> AppState {

    dotenv().ok();

    let config = Config::from_env().unwrap();

    let pool = config.pg.create_pool(NoTls).unwrap();

    let log = Config::configure_log();

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
        .route("/", web::get().to(todos));

    let mut app = test::init_service(app).await;

    let req = test::TestRequest::get().uri("/").to_request();

    let todos: Vec<TodoList> = test::read_response_json(&mut app, req).await;

    info!(log, "{:?}", todos);

}
