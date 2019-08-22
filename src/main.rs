#[macro_use] extern crate validator_derive;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate diesel;

use std::io;
use actix_web::{
    web, App, HttpResponse, HttpServer, Error as AWError, middleware, error::ResponseError
};
use futures::future::Future;
use std::path::Path;
use bcrypt::{hash, verify};
use listenfd::ListenFd;

pub mod db;
pub mod utils;
pub mod schema;
pub mod errors;

fn index() -> HttpResponse {
    HttpResponse::Ok().body("Junto Holochain Alpha API")
}

fn register(data: web::Json<db::models::RegisterData>, pool: web::Data<db::Pool>) -> impl Future<Item = HttpResponse, Error = AWError>  {
    web::block(move || {
        let pub_key = utils::holochain::assign_agent(&pool)?;
        let id = uuid::Uuid::new_v4();
        let hashed_password = hash(data.password.clone(), 13).map_err(|_err| errors::JuntoApiError::InternalError)?;
        let user = db::models::Users{id: id.clone(), email: data.email.clone(), password: hashed_password, pub_address: pub_key, 
                    first_name: data.first_name.clone(), last_name: data.last_name.clone()};
        db::models::Users::insert_user(&user, &pool).map_err(|_err| errors::JuntoApiError::InternalError)?;
        let token = utils::jwt::Token::create(format!("{}", id)).map_err(|_err| errors::JuntoApiError::InternalError)?;
        Ok(token)
    })
    .then(|token: Result<String, actix_threadpool::BlockingError<errors::JuntoApiError>>| match token {
        Ok(token) => Ok(HttpResponse::Ok().json(json!({"token": token}))),
        Err(err) => match err {
                        actix_threadpool::BlockingError::Error(err) => {
                            Ok(err.error_response())
                        },
                        actix_threadpool::BlockingError::Canceled => {
                            Ok(HttpResponse::InternalServerError().into())
                        }
                    },
    })
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
            .route("/register", web::post().to_async(register))
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
