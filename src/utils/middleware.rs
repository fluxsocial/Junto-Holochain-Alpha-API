// use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error as AWError, error::ResponseError};
// use actix_service::{Service, Transform};
// use futures::future::{ok, FutureResult};
// use futures::{Future, Poll};

// use crate::utils::jwt;
// use crate::errors;

// struct Auth;

// // Middleware factory is `Transform` trait from actix-service crate
// // `S` - type of the next service
// // `B` - type of response's body
// impl<S, B> Transform<S> for Auth
// where
//     S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = AWError>,
//     S::Future: 'static,
//     B: 'static,
// {
//     type Request = ServiceRequest;
//     type Response = ServiceResponse<B>;
//     type Error = AWError;
//     type InitError = ();
//     type Transform = AuthMiddleware<S>;
//     type Future = FutureResult<Self::Transform, Self::InitError>;

//     fn new_transform(&self, service: S) -> Self::Future {
//         ok(AuthMiddleware{service: service})
//     }
// }

// pub struct AuthMiddleware<S> {
//     service: S,
// }

// impl<S, B> Service for AuthMiddleware<S>
// where
//     S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = AWError>,
//     S::Future: 'static,
//     B: 'static,
// {
//     type Request = ServiceRequest;
//     type Response = ServiceResponse<B>;
//     type Error = AWError;
//     type Future = Box<dyn Future<Item = Self::Response, Error = Self::Error>>;

//     fn poll_ready(&mut self) -> Poll<(), Self::Error> {
//         self.service.poll_ready()
//     }

//     fn call(&mut self, req: ServiceRequest) -> Self::Future {
//         if req.method() == "OPTIONS" {
//             return Box::new(self.service.call(req).and_then(|res| {
//                 println!("Hi from response");
//                 Ok(res)
//             }));
//         };

//         // read the AUTH header from the request
//         let token = req.headers()
//             .get("AUTH")
//             .ok_or(errors::JuntoApiError::Unauthorized);

//         match token {
//             Ok(t) => {
//                 // check that the token is valid - up to you how you do this
//                 match jwt::Token::verify(t.to_str().unwrap()){
//                     Ok(String) => {
//                         Box::new(self.service.call(req).and_then(|res| {
//                             println!("Hi from response");
//                             Ok(res)
//                         }))
//                     },
//                     Err(_err) => Box::new(Ok(req.error_response(AWError::from(errors::JuntoApiError::Unauthorized))))
//                 }
//             },
//             Err(_err) => Box::new(Ok(req.error_response(AWError::from(errors::JuntoApiError::Unauthorized))))
//         }
//     }
// }