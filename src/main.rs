#[macro_use]
extern crate actix_web;
extern crate actix_rt;

use actix_web::{Responder, HttpServer, App};
use std::io::Result;


#[get("/")]
async fn hello() -> impl Responder {
    format!("Hello World!")
}


#[actix_rt::main]
async fn main() -> Result<()> {
    println!("Starting server at http://127.0.0.1:8080");
    HttpServer::new(|| App::new() 
        .service(hello)
    )
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
