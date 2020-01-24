use actix_web::{Responder, HttpServer, App, web, get};
use serde::Serialize;
use std::io::Result;

#[derive(Serialize)]
struct Status {
    status: String
}

#[get("/")]
async fn hello() -> impl Responder {
    web::HttpResponse::Ok()
        .json(Status {status: "Up".to_string()})
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
