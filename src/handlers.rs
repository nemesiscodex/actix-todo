use crate::db;
use crate::models::{AppState, CreateTodoItem, CreateTodoList, ResultResponse, Status};

use crate::errors::AppError;
use actix_web::{web, HttpResponse, Responder};
use deadpool_postgres::{Client, Pool, PoolError};
use slog::{crit, error, o, Logger};

async fn get_client(pool: Pool, log: Logger) -> Result<Client, AppError> {
    pool.get().await.map_err(|err: PoolError| {
        let sublog = log.new(o!("cause" => err.to_string()));
        crit!(sublog, "Error creating client");
        AppError::from(err)
    })
}

fn log_error(log: Logger) -> impl Fn(AppError) -> AppError {
    move |err| {
        let log = log.new(o!(
            "cause" => err.cause.clone()
        ));
        error!(log, "{}", err.message());
        err
    }
}
pub async fn status() -> Result<impl Responder, AppError> {
    Ok(web::HttpResponse::Ok().json(Status {
        status: "Up".to_string(),
    }))
}

pub async fn todos(state: web::Data<AppState>) -> Result<impl Responder, AppError> {
    let sublog = state.log.new(o!("handler" => "create_todo"));

    let client: Client = get_client(state.pool.clone(), sublog.clone()).await?;

    let result = db::get_todos(&client).await;

    result
        .map(|todos| HttpResponse::Ok().json(todos))
        .map_err(log_error(sublog))
}

pub async fn create_todo(
    todo_list: web::Json<CreateTodoList>,
    state: web::Data<AppState>,
) -> Result<impl Responder, AppError> {
    let sublog = state.log.new(o!(
        "handler" => "create_todo",
        "todo_list" => todo_list.title.clone()
    ));

    let client: Client = get_client(state.pool.clone(), sublog.clone()).await?;

    let result = db::create_todo(&client, todo_list.title.clone()).await;

    result
        .map(|todo| HttpResponse::Ok().json(todo))
        .map_err(log_error(sublog))
}

pub async fn get_todo(
    list_id: web::Path<(i32,)>,
    state: web::Data<AppState>,
) -> Result<impl Responder, AppError> {
    let sublog = state.log.new(o!(
        "handler" => "get_todo",
        "list_id" => list_id.0
    ));

    let client: Client = get_client(state.pool.clone(), sublog.clone()).await?;

    let result = db::get_todo(&client, list_id.0).await;

    result
        .map(|todo| HttpResponse::Ok().json(todo))
        .map_err(log_error(sublog))
}
pub async fn create_item(
    list_id: web::Path<(i32,)>,
    todo_item: web::Json<CreateTodoItem>,
    state: web::Data<AppState>,
) -> Result<impl Responder, AppError> {
    let sublog = state.log.new(o!(
        "handler" => "create_item",
        "list_id" => list_id.0,
        "todo_item" => todo_item.title.clone()
    ));

    let client: Client = get_client(state.pool.clone(), sublog.clone()).await?;

    let result = db::create_item(&client, list_id.0, todo_item.title.clone()).await;

    result
        .map(|item| HttpResponse::Ok().json(item))
        .map_err(log_error(sublog))
}

pub async fn items(
    list_id: web::Path<(i32,)>,
    state: web::Data<AppState>,
) -> Result<impl Responder, AppError> {
    let sublog = state.log.new(o!(
        "handler" => "items",
        "list_id" => list_id.0
    ));

    let client: Client = get_client(state.pool.clone(), sublog.clone()).await?;

    let result = db::get_items(&client, list_id.0).await;

    result
        .map(|items| HttpResponse::Ok().json(items))
        .map_err(log_error(sublog))
}

pub async fn get_item(
    params: web::Path<(i32, i32)>,
    state: web::Data<AppState>,
) -> Result<impl Responder, AppError> {
    let sublog = state.log.new(o!(
        "handler" => "get_item",
        "list_id" => params.0,
        "item_id" => params.1,
    ));

    let client: Client = get_client(state.pool.clone(), sublog.clone()).await?;

    let result = db::get_item(&client, params.0, params.1).await;

    result
        .map(|item| HttpResponse::Ok().json(item))
        .map_err(log_error(sublog))
}

pub async fn check_todo(
    params: web::Path<(i32, i32)>,
    state: web::Data<AppState>,
) -> Result<impl Responder, AppError> {
    let sublog = state.log.new(o!(
        "handler" => "check_todo",
        "list_id" => params.0,
        "item_id" => params.1,
    ));

    let client: Client = get_client(state.pool.clone(), sublog.clone()).await?;

    let result = db::check_todo(&client, params.0, params.1).await;

    result
        .map(|updated| HttpResponse::Ok().json(ResultResponse { result: updated }))
        .map_err(log_error(sublog))
}
