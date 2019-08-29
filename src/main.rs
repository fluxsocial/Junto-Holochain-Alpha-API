#[macro_use] extern crate validator_derive;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate diesel;

use std::io;
use actix_web::{
    web, App, HttpResponse, HttpServer, middleware
};
use actix_identity::{CookieIdentityPolicy, IdentityService, Identity};
use std::path::Path;
use listenfd::ListenFd;

pub mod models;
pub mod utils;
pub mod schema;
pub mod errors;
pub mod handlers;

fn index() -> HttpResponse {
    HttpResponse::Ok().body("Junto Holochain Alpha API")
}

fn test(id: Identity) -> HttpResponse {
    println!("Identity of request {:?}", id.identity());
    HttpResponse::Ok().body("Test response")
}

fn main() -> io::Result<()> {
    let path = Path::new("./src/development.env");
    dotenv::from_path(&path).expect("Unable to load .env");
    
    let sys = actix_rt::System::new("api");
    let pool = utils::load_connection_pool();
    let domain: String = std::env::var("DOMAIN").unwrap_or_else(|_| "localhost".to_string());
    let mut listenfd = ListenFd::from_env();

    let mut server = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new("supersupersupersecretsecretkey...".as_bytes())
                    .name("auth")
                    .path("/")
                    .domain(domain.as_str())
                    .max_age_time(chrono::Duration::days(1))
                    .secure(false), // this can only be true if you have https
            ))
            .data(pool.clone())
            .route("/", web::get().to(index))
            .route("/test", web::get().to(test))
            .route("/register", web::post().to_async(handlers::auth::register))
            .service(
                web::resource("/auth")
                    .route(web::post().to_async(handlers::auth::login))
                    .route(web::delete().to(handlers::auth::logout))
                    .route(web::get().to(handlers::auth::get_me))
            )
            .route("/holochain", web::post().to_async(handlers::holochain::holochain))
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
