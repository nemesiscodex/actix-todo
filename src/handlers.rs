use crate::models::{CreateTodoList, CreateTodoItem, Status, ResultResponse};
use crate::db;

use actix_web::{web, Responder, HttpResponse};
use deadpool_postgres::{Pool, Client};
use crate::errors::AppError;

pub async fn status() -> Result<impl Responder, AppError> {
    Ok(web::HttpResponse::Ok()
        .json(Status {status: "Up".to_string()}))
}

pub async fn todos(db_pool: web::Data<Pool>) -> Result<impl Responder, AppError> {

    let client: Client =
        db_pool.get().await.map_err(|err| AppError::DbError(err.to_string()))?;

    let result = db::get_todos(&client).await;

    result.map(|todos| HttpResponse::Ok().json(todos))
}

pub async fn create_todo(todo_list: web::Json<CreateTodoList>,db_pool: web::Data<Pool>) -> Result<impl Responder, AppError> {

    let client: Client = 
        db_pool.get().await.map_err(|err| AppError::DbError(err.to_string()))?;

    let result = db::create_todo(&client, todo_list.title.clone()).await;

    result.map(|todo| HttpResponse::Ok().json(todo))
}

pub async fn get_todo(list_id: web::Path<(i32,)>, db_pool: web::Data<Pool>) -> Result<impl Responder, AppError> {

    let client: Client = 
        db_pool.get().await.map_err(|err| AppError::DbError(err.to_string()))?;

    let result = db::get_todo(&client, list_id.0).await;

    result.map(|todo| HttpResponse::Ok().json(todo))

}
pub async fn create_item(list_id: web::Path<(i32,)>, todo_item: web::Json<CreateTodoItem>, db_pool: web::Data<Pool>) -> Result<impl Responder, AppError> {

    let client: Client = 
        db_pool.get().await.map_err(|err| AppError::DbError(err.to_string()))?;

    let result = db::create_item(&client, list_id.0, todo_item.title.clone()).await;

    result.map(|item| HttpResponse::Ok().json(item))
}

pub async fn items(list_id: web::Path<(i32,)>, db_pool: web::Data<Pool>) -> Result<impl Responder, AppError> {

    let client: Client =
        db_pool.get().await.map_err(|err| AppError::DbError(err.to_string()))?;

    let result = db::get_items(&client, list_id.0).await;

    result.map(|items| HttpResponse::Ok().json(items))
}


pub async fn get_item(params: web::Path<(i32,i32)>, db_pool: web::Data<Pool>) -> Result<impl Responder, AppError> {

    let client: Client = 
        db_pool.get().await.map_err(|err| AppError::DbError(err.to_string()))?;

    let result = db::get_item(&client, params.0, params.1).await;

    result.map(|item| HttpResponse::Ok().json(item))

}


pub async fn check_todo(params: web::Path<(i32,i32)>, db_pool: web::Data<Pool>) -> Result<impl Responder, AppError> {

    let client: Client = 
        db_pool.get().await.map_err(|err| AppError::DbError(err.to_string()))?;

    let result = db::check_todo(&client, params.0, params.1).await;

    result.map(|_| HttpResponse::Ok().json(ResultResponse{result: true}))

}