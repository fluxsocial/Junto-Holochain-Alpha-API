// fn holochain(data: web::Json<db::models::HolochainUserRequst>, pool: web::Data<db::Pool>, req: HttpRequest) 
//     -> impl Future<Item = HttpResponse, Error = AWError> {
//     let token = utils::jwt::Token::verify_request_headers(req)?;
//     web::block(move ||{
//         println!("User ID: {:?}", token);
//         let pub_address = db::models::Users::get_pub_key(&token, &pool)?;
//         let holochain_call = utils::holochain::call_holochain(&data, pub_address)?;
//         Ok(holochain_call)
//     })
//     .then(|holochain_result: Result<db::models::HolochainResponse, actix_threadpool::BlockingError<errors::JuntoApiError>>| match holochain_result {
//         Ok(holochain_result) => Ok(HttpResponse::Ok().json(holochain_result)),
//         Err(err) => match err{
//                         actix_threadpool::BlockingError::Error(err) => {
//                             Ok(err.error_response())
//                         },
//                         actix_threadpool::BlockingError::Canceled => {
//                             Ok(HttpResponse::InternalServerError().into())
//                         }
//                     }
//     })
// }