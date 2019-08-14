#[macro_use] extern crate validator_derive;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;

use std::io;
use actix_web::{
    web, App, HttpResponse, HttpServer, Error, middleware
};
use diesel::r2d2::ConnectionManager;
use diesel::pg::PgConnection;
use futures::future::{Future, ok};
use std::env;
use std::path::Path;

mod db;
mod token;
mod schema;

fn register(data: web::Json<db::models::RegisterData>, pool: web::Data<db::Pool>) -> Box<dyn Future<Item = HttpResponse, Error = Error>> {
    Box::new(ok::<_, Error>(
        HttpResponse::Ok().content_type("text/html").body("Hello!"),
    ))
}

fn main() -> io::Result<()> {
    let path = Path::new("./src/development.env");
    dotenv::from_path(&path).expect("Unable to load .env");
    let sys = actix_rt::System::new("api");

    let database_url = env::var("DATABASE_URL").unwrap_or_else(|e| {
        panic!("could not find {}: {}", "DATABASE_URL", e)
    });
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = db::Pool::new(manager).unwrap();

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .route("/register", web::to_async(register))
    })
    .bind("127.0.0.1:8080")?
    .start();

    println!("Started http server: 127.0.0.1:8080");
    sys.run()
}
