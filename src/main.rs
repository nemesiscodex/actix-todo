mod models;
mod db;

use actix_web::{Responder, HttpServer, App, web, get, Error};
use actix::Addr;
use std::io;
use serde::Serialize;
use crate::db::{PgConnection, GetTodoLists, GetTodoList};

#[derive(Serialize)]
struct Status {
    status: String
}

#[get("/")]
async fn status() -> impl Responder {
    web::HttpResponse::Ok()
        .json(Status {status: "Up".to_string()})
}

async fn todos(db: web::Data<Addr<PgConnection>>) -> Result<web::HttpResponse, Error> {
    let res = db.send(GetTodoLists).await?;

    match res {
        Ok(lists) => Ok(web::HttpResponse::Ok().json(lists)),
        Err(_) => Ok(web::HttpResponse::InternalServerError().into())
    }
}

async fn todo(db: web::Data<Addr<PgConnection>>, id: web::Path<(i32,)>) -> Result<web::HttpResponse, Error> {
    let res = db.send(GetTodoList{id:id.0}).await?;

    match res {
        Ok(lists) => Ok(web::HttpResponse::Ok().json(lists)),
        Err(_) => Ok(web::HttpResponse::InternalServerError().into())
    }
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    println!("Starting server at http://127.0.0.1:8080");
    
    const DB_URL: &str = "postgres://actix:actix@localhost:5432/actix";

    HttpServer::new(|| App::new()
        .data_factory(|| PgConnection::connect(DB_URL)) 
        .service(status)
        .service(web::resource("/todos").to(todos))
        .service(web::resource("/todos/{id}").to(todo))
    )
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
