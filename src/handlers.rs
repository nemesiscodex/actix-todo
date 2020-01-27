use crate::models::{CreateTodoList, CreateTodoItem, Status, ResultResponse};
use crate::db;
use std::io;

use actix_web::{web, Responder, HttpResponse};
use deadpool_postgres::{Pool, Client};
use io::ErrorKind::{NotFound, Other};

pub async fn status() -> impl Responder {
    web::HttpResponse::Ok()
        .json(Status {status: "Up".to_string()})
}

pub async fn todos(db_pool: web::Data<Pool>) -> impl Responder {

    let client: Client =
        db_pool.get().await.expect("Error connecting to the database");

    let result = db::get_todos(&client).await;

    match result {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(_) => HttpResponse::InternalServerError().into()
    }
}

pub async fn create_todo(todo_list: web::Json<CreateTodoList>,db_pool: web::Data<Pool>) -> impl Responder {

    let client: Client = 
        db_pool.get().await.expect("Error connecting to the database");

    let result = db::create_todo(&client, todo_list.title.clone()).await;

    match result {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(_) => HttpResponse::InternalServerError().into()
    }
}

pub async fn get_todo(list_id: web::Path<(i32,)>, db_pool: web::Data<Pool>) -> impl Responder {

    let client: Client = 
        db_pool.get().await.expect("Error connecting to the database");

    let result = db::get_todo(&client, list_id.0).await;

    match result {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(ref e) if e.kind() == NotFound => HttpResponse::NotFound().into(),
        Err(_) => HttpResponse::InternalServerError().into()
    }

}
pub async fn create_item(list_id: web::Path<(i32,)>, todo_item: web::Json<CreateTodoItem>, db_pool: web::Data<Pool>) -> impl Responder {

    let client: Client = 
        db_pool.get().await.expect("Error connecting to the database");

    let result = db::create_item(&client, list_id.0, todo_item.title.clone()).await;

    match result {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(_) => HttpResponse::InternalServerError().into()
    }
}

pub async fn items(list_id: web::Path<(i32,)>, db_pool: web::Data<Pool>) -> impl Responder {

    let client: Client =
        db_pool.get().await.expect("Error connecting to the database");

    let result = db::get_items(&client, list_id.0).await;

    match result {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(_) => HttpResponse::InternalServerError().into()
    }
}


pub async fn get_item(params: web::Path<(i32,i32)>, db_pool: web::Data<Pool>) -> impl Responder {

    let client: Client = 
        db_pool.get().await.expect("Error connecting to the database");

    let result = db::get_item(&client, params.0, params.1).await;

    match result {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(ref e) if e.kind() == NotFound => HttpResponse::NotFound().into(),
        Err(_) => HttpResponse::InternalServerError().into()
    }

}


pub async fn check_todo(params: web::Path<(i32,i32)>, db_pool: web::Data<Pool>) -> impl Responder {

    let client: Client = 
        db_pool.get().await.expect("Error connecting to the database");

    let result = db::check_todo(&client, params.0, params.1).await;

    match result {
        Ok(()) => HttpResponse::Ok().json(ResultResponse{result:true}),
        Err(ref e) if e.kind() == Other =>  HttpResponse::Ok().json(ResultResponse{result:false}),
        Err(_) => HttpResponse::InternalServerError().into()
    }

}