use actix_web::body::{BoxBody, MessageBody};
use actix_web::http::{self, StatusCode};
use actix_web::{web, HttpMessage, HttpResponse, HttpResponseBuilder};
use actix_web::dev::{ServiceRequest, Transform, forward_ready};
use actix_web::{dev::Service, dev::ServiceResponse, Error};
use diesel::insert_into;
use std::future::{ready, Ready};
use std::pin::Pin;

use crate::common::app_error::{AppError, AppErrorType};
use crate::common::app_http_response_builder::AppHttpResponseBuilder;
use crate::{common, RequestExtension};
use crate::utils::jwt_helpers::{validate_jwt, Claims};


pub struct ValidateJwtMiddleware;

// impl<S, B> Transform<S, ServiceRequest> for ValidateJwtMiddleware
impl<S> Transform<S, ServiceRequest> for ValidateJwtMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error>,
    // S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    // S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = AppError>,
    S::Future: 'static,
    // B: 'static,
{
    type Response = ServiceResponse<BoxBody>;
    // type Response = ServiceResponse<B>;
    type Error = Error;
    // type Error = AppError;
    type InitError = ();
    type Transform = ValidateJwtMiddlewareMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ValidateJwtMiddlewareMiddleware { service }))
    }
}

// Important docs: https://www.shuttle.rs/blog/2023/12/15/using-actix-rust

pub struct ValidateJwtMiddlewareMiddleware<S> {
    service: S,
}


impl From<actix_web::Error> for AppError {
    fn from(err: actix_web::Error) -> Self {
        // Convert actix_web::Error into your AppError
        return AppError::new(AppErrorType::InvalidToken);
    }
}


// impl<S, B> Service<ServiceRequest> for ValidateJwtMiddlewareMiddleware<S>
impl<S> Service<ServiceRequest> for ValidateJwtMiddlewareMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error>,
    // S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    // S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = AppError>,
    S::Future: 'static,
    // B: 'static,
{
    // type Response = ServiceResponse<B>;
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    // type Error = AppError;
    type Future = Pin<Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>> + 'static>>;

    forward_ready!(service);

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let headers = req.headers().clone();
        
        // TODO: manage this errors correctly instead of using expect
        let token = headers.get("token").expect("No token was received").to_str().expect("Error in token parsing");
        // let jwt_payload: Claims = validate_jwt(common::config::CONFIG_OBJECT.token.basic_token_secret.as_str(), token).expect("Error in token decoding");
        
        let jwt_payload_result = validate_jwt(common::config::CONFIG_OBJECT.token.basic_token_secret.as_str(), token);
        let jwt_payload;

        if let Err(jwt_error) = jwt_payload_result {
            println!("Error in jwt validation: {}", jwt_error);
            // let error = AppError::new(AppErrorType::InvalidToken);
            // let asdasd = AppHttpResponseBuilder::get_http_response::<()>(Err(error));

            // let actix_error: actix_web::Error = asdasd.into();

            // // // return Box::pin(async { Ok(req.into_response(AppHttpResponseBuilder::get_http_response(Err(error)))) });
            // // let response = AppHttpResponseBuilder::get_http_response(Err(error)).map_into_boxed_body().map_into_right_body::<B>();
            // let response: HttpResponse<BoxBody> = AppHttpResponseBuilder::get_http_response(Err(error)).map_into_boxed_body().map_into_right_body::<B>();
            // return Box::pin(async { Ok(req.into_response(response)) });

            let mut err = actix_web::error::ErrorInternalServerError("Something went wrong!");

            let bb = err.as_response_error();

            // Create an actix_web::Error instance
            let actix_error: actix_web::Error = err.into();

            // return Box::pin({Ok(req.into_response( 
            //     HttpResponse::Unauthorized()
            //         .finish().map_into_boxed_body()
            // ))});

            // let error = AppError::new(AppErrorType::InvalidToken);
            // let response = AppHttpResponseBuilder::get_http_response::<B>(Err(error)).map_into_boxed_body();
            // return Box::pin(async { Ok(ServiceResponse::new(req.request().clone(), response)) });
        


            // return Box::pin(async {Ok(ServiceResponse::new(
			// 	req.request().clone(),
			// 	HttpResponseBuilder::new(req.status()).body("body test"),
			// ))})

            // let early_response = HttpResponse::Ok()
            // .content_type("text/plain")
            // .body("Early response body");

            // let early_response = HttpResponse::with_body(StatusCode::FORBIDDEN, B {a: "buenas"})

            // if (true) {
            //     let (request, _pl) = req.into_parts();

            //     // let response: HttpResponse<B> = HttpResponse::Found()
            //     let response = HttpResponse::Found()
            //     .insert_header((http::header::LOCATION, "/login"))
            //     .finish()
            //     // constructed responses map to "right" body
            //     // .map_into_right_body()
            //     ;

                

            //     return Box::pin(async { Ok(ServiceResponse::new(request, response)) });
            // }

            let (request, _pl) = req.into_parts();

            // let response: HttpResponse<B> = HttpResponse::Found()
            // let response = HttpResponse::Found()
            let response = HttpResponse::Found()
            .insert_header((http::header::LOCATION, "/login"))
            .finish()
            // constructed responses map to "right" body
            // .map_into_right_body()
            ;

            

            return Box::pin(async { Ok(ServiceResponse::new(request, response)) });


            // // Create a ServiceResponse instance with the early response
            // // let service_response = ServiceResponse::new(req.into_parts().0, early_response.map_into_boxed_body());
            // // let service_response = ServiceResponse::new(req.into_parts().0, early_response.map_into_boxed_body().map_into_right_body::<B>());
            // // let aux_ekisde = early_response.map_into_right_body::<B>();
            // let aux_ekisde = early_response.map_into_right_body::<B>();
            // // let service_response = ServiceResponse::new(req.into_parts().0, early_response.map_into_right_body::<B>());
            // let service_response = ServiceResponse::new(req.into_parts().0, early_response.map_into_right_body::<B>());

            // let asasas = service_response.;

            // return Box::pin(async move {
            //     // Return the early ServiceResponse
            //     Ok(service_response)
            // });


            // return Box::pin(async { Err(actix_error) });
            // // return Box::pin(async { Err(error) });

        } else {
            jwt_payload = jwt_payload_result.unwrap()
        }


        // let jwt_payload = match jwt_payload {
        //     Ok(payload) => payload,
        //     Err(_) => {
        //         let response = HttpResponse::Unauthorized()
        //             .json("Invalid token");
        //         return Box::pin(async { Ok(req.into_response(response.into_body())) });
        //     }
        // };

        {
            let mut extensions = req.extensions_mut();
            let extensions_value = extensions.get::<RequestExtension>().expect("The extension was never initialized");
            let mut cloned_extension = extensions_value.clone();
            cloned_extension.jwt_payload = Some(jwt_payload);
            extensions.insert(cloned_extension);
            // println!("Extensions asdasdasdasdasd: {:?}", req.extensions().get::<RequestExtension>());
        }
        
        let fut = self.service.call(req);

        Box::pin(async move {
            println!("Hi from jwt");
            let res = fut.await?;
            println!("RIGHT BEFORE JWT OK(RES): {:?}", res.status());
            Ok(res)
        })
    }
}
