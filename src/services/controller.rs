use super::service::*;
use actix_web::{ Responder, web, HttpResponse, http::StatusCode};
use crate::{services::dto::*, shared::{TakTikResponse, ResResult, ResError}};
use actix_web::HttpResponseBuilder;
use validator::Validate;

pub async fn get_service(path: web::Path<GetOneService>) -> impl Responder {
    match path.validate() {
        Ok(_result) => {
            let my_service = get_one_service(path.id).await;
            // HttpResponse::Ok().json(my_service)
            HttpResponseBuilder::new(my_service.status).json(my_service)
        },
        Err(err) => HttpResponse::BadRequest().json(TakTikResponse {
            error: true,
            status: StatusCode::BAD_REQUEST,
            result: ResResult::<()>::Error(ResError::ValidationErrors(err))
        })
    }
    
}

pub async fn get() -> impl Responder {
    let my_services = get_services_service().await;
    HttpResponseBuilder::new(my_services.status).json(my_services)
}

pub async fn create(form: web::Json<UploadForm>) -> impl Responder {
    match form.validate() {
        Ok(_result) => {
            let my_service = create_service(form.into_inner()).await;
            HttpResponseBuilder::new(my_service.status).json(my_service)
        },
        Err(err) => HttpResponse::BadRequest().json(TakTikResponse {
            error: true,
            status: StatusCode::BAD_REQUEST,
            result: ResResult::<()>::Error(ResError::ValidationErrors(err))
        })
    }
}
