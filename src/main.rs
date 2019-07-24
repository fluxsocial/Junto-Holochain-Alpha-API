extern crate dotenv;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use std::env;

fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello Jessy you have a cute mouth!")
}

fn index2() -> impl Responder {
    HttpResponse::Ok().body("Hello world again!")
}

fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|e| {
        panic!("could not find {}: {}", "DATABASE_URL", e)
    });
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/again", web::get().to(index2))
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap();
}
