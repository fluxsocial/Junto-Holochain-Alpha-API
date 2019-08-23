#[macro_use] extern crate validator_derive;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate diesel;

use std::io;
use actix_web::{
    web, App, HttpResponse, HttpServer, middleware
};
use std::path::Path;
use listenfd::ListenFd;

pub mod db;
pub mod utils;
pub mod schema;
pub mod errors;
pub mod handlers;

fn index() -> HttpResponse {
    HttpResponse::Ok().body("Junto Holochain Alpha API")
}

fn main() -> io::Result<()> {
    let path = Path::new("./src/development.env");
    dotenv::from_path(&path).expect("Unable to load .env");
    
    let sys = actix_rt::System::new("api");
    let pool = utils::load_connection_pool();
    let mut listenfd = ListenFd::from_env();

    let mut server = HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(index))
            .route("/register", web::post().to_async(handlers::auth::register))
            // .route("/holochain", web::post().to_async(holochain))
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l).unwrap()
    } else {
        server.bind("127.0.0.1:8080").unwrap()
    };
    server.start();

    println!("Started http server: 127.0.0.1:8080");
    sys.run()
}
