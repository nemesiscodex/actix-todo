// File: src/handlers.rs
// High-level: HTTP endpoint handlers. Each handler validates inputs, acquires a DB client from the pool,
// delegates to the data-access layer, and maps results/errors to HTTP responses with structured logging.
use crate::db;
use crate::models::{AppState, CreateTodoItem, CreateTodoList, ResultResponse, Status};

use crate::errors::AppError;
use actix_web::{web, HttpResponse, Responder};
use deadpool_postgres::{Client, Pool, PoolError};
use slog::{crit, error, o, Logger};

// Acquire a client from the shared pool. We log the pool error details but return a generic HTTP error.
async fn get_client(pool: &Pool, log: &Logger) -> Result<Client, AppError> {
    pool.get().await.map_err(|err: PoolError| {
        let sublog = log.new(o!("cause" => err.to_string()));
        crit!(sublog, "Error creating client");
        AppError::from(err)
    })
}

// Convert an AppError into a logged error, preserving the original error for Actix to render.
fn log_error(log: Logger) -> impl Fn(AppError) -> AppError {
    move |err| {
        let log = log.new(o!(
            "cause" => err.cause.clone()
        ));
        error!(log, "{}", err.message());
        err
    }
}
// Simple readiness endpoint so clients (and tests) can verify the service is up.
pub async fn status() -> Result<impl Responder, AppError> {
    Ok(HttpResponse::Ok().json(Status {
        status: "Up".to_string(),
    }))
}

// List all todo lists. No input required.
pub async fn todos(state: web::Data<AppState>) -> Result<impl Responder, AppError> {
    let sublog = state.log.new(o!("handler" => "create_todo"));

    let client: Client = get_client(&state.pool, &sublog).await?;

    let result = db::get_todos(&client).await;

    result
        .map(|todos| HttpResponse::Ok().json(todos))
        .map_err(log_error(sublog))
}

// Create a new todo list. Validates JSON payload via serde.
pub async fn create_todo(
    todo_list: web::Json<CreateTodoList>,
    state: web::Data<AppState>,
) -> Result<impl Responder, AppError> {
    let title = todo_list.into_inner().title;
    let sublog = state
        .log
        .new(o!("handler" => "create_todo", "todo_list" => title.clone()))
        ;

    let client: Client = get_client(&state.pool, &sublog).await?;

    let result = db::create_todo(&client, &title).await;

    result
        .map(|todo| HttpResponse::Ok().json(todo))
        .map_err(log_error(sublog))
}

// Fetch a specific todo list by id.
pub async fn get_todo(
    list_id: web::Path<(i32,)>,
    state: web::Data<AppState>,
) -> Result<impl Responder, AppError> {
    let sublog = state.log.new(o!(
        "handler" => "get_todo",
        "list_id" => list_id.0
    ));

    let client: Client = get_client(&state.pool, &sublog).await?;

    let result = db::get_todo(&client, list_id.0).await;

    result
        .map(|todo| HttpResponse::Ok().json(todo))
        .map_err(log_error(sublog))
}
// Create a new item in a given list.
pub async fn create_item(
    list_id: web::Path<(i32,)>,
    todo_item: web::Json<CreateTodoItem>,
    state: web::Data<AppState>,
) -> Result<impl Responder, AppError> {
    let title = todo_item.into_inner().title;
    let sublog = state
        .log
        .new(o!("handler" => "create_item", "list_id" => list_id.0, "todo_item" => title.clone()));

    let client: Client = get_client(&state.pool, &sublog).await?;

    let result = db::create_item(&client, list_id.0, &title).await;

    result
        .map(|item| HttpResponse::Ok().json(item))
        .map_err(log_error(sublog))
}

// List items in a given todo list.
pub async fn items(
    list_id: web::Path<(i32,)>,
    state: web::Data<AppState>,
) -> Result<impl Responder, AppError> {
    let sublog = state.log.new(o!(
        "handler" => "items",
        "list_id" => list_id.0
    ));

    let client: Client = get_client(&state.pool, &sublog).await?;

    let result = db::get_items(&client, list_id.0).await;

    result
        .map(|items| HttpResponse::Ok().json(items))
        .map_err(log_error(sublog))
}

// Fetch a specific item given list and item ids.
pub async fn get_item(
    params: web::Path<(i32, i32)>,
    state: web::Data<AppState>,
) -> Result<impl Responder, AppError> {
    let sublog = state.log.new(o!(
        "handler" => "get_item",
        "list_id" => params.0,
        "item_id" => params.1,
    ));

    let client: Client = get_client(&state.pool, &sublog).await?;

    let result = db::get_item(&client, params.0, params.1).await;

    result
        .map(|item| HttpResponse::Ok().json(item))
        .map_err(log_error(sublog))
}

// Mark a todo item as checked. Idempotent at DB layer; returns whether a row was updated.
pub async fn check_todo(
    params: web::Path<(i32, i32)>,
    state: web::Data<AppState>,
) -> Result<impl Responder, AppError> {
    let sublog = state.log.new(o!(
        "handler" => "check_todo",
        "list_id" => params.0,
        "item_id" => params.1,
    ));

    let client: Client = get_client(&state.pool, &sublog).await?;

    let result = db::check_todo(&client, params.0, params.1).await;

    result
        .map(|updated| HttpResponse::Ok().json(ResultResponse { result: updated }))
        .map_err(log_error(sublog))
}
