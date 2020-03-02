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
mod integration_tests {
    
    use actix_web::{test, App, web};
    use slog::info;
    use dotenv::dotenv;
    use crate::config::Config;
    use crate::models;
    use crate::handlers;
    use tokio_postgres::NoTls;
    use lazy_static::lazy_static;
    use serde_json::json;


    lazy_static! {
        static ref APP_STATE: models::AppState = {
    
            dotenv().ok();
        
            let log = Config::configure_log();

            info!(log, "Creating static AppState");
        
            let config = Config::from_env().unwrap();
        
            let pool = config.pg.create_pool(NoTls).unwrap();
        
            models::AppState {
                pool: pool.clone(),
                log: log.clone()
            }
        
        };
    }

    #[actix_rt::test]
    async fn test_get_todos() {
    
        let todos_path = "/todos/";

        let app = App::new()
            .data(APP_STATE.clone())
            .route(todos_path, web::get().to(handlers::todos));
    
        let mut app = test::init_service(app).await;
    
        let req = test::TestRequest::get()
            .uri(todos_path)
            .to_request();
    
        let response = test::call_service(&mut app, req).await;
    
        assert_eq!(response.status(), 200, "GET /todos should return 200");
    
        let body = test::read_body(response).await;
    
        let try_todos: Result<Vec<models::TodoList>, serde_json::error::Error> = serde_json::from_slice(&body);
    
        assert!(try_todos.is_ok(), "Response couldn't not be parsed");
    
    }
    
    #[actix_rt::test]
    async fn test_create_todos() {
    
        let todos_path = "/todos/";
    
        let app = App::new()
            .data(APP_STATE.clone())
            .route(todos_path, web::post().to(handlers::create_todo))
            .route(todos_path, web::get().to(handlers::todos));
    
    
        let mut app = test::init_service(app).await;
    
        let todo_title = "Create todo List";
    
        let create_todo_list = json!({"title": todo_title});
    
        let req = test::TestRequest::post()
            .uri(todos_path)
            .header("Content-Type", "application/json")
            .set_payload(create_todo_list.to_string())
            .to_request();
    
        let response = test::call_service(&mut app, req).await;
    
        assert_eq!(response.status(), 200, "Status should be 200.");
    
        let body = test::read_body(response).await;
    
        let try_created: Result<models::TodoList, serde_json::error::Error> = serde_json::from_slice(&body);
    
        assert!(try_created.is_ok(), "Response couldn't not be parsed");
    
        let created_list = try_created.unwrap();
    
        let req = test::TestRequest::get()
            .uri(todos_path)
            .to_request();
    
        let todos: Vec<models::TodoList> = test::read_response_json(&mut app, req).await;
    
        let maybe_list = todos
            .iter()
            .find(|todo| todo.id == created_list.id);
    
        assert!(maybe_list.is_some(), "Item not found!");
    
    }
}