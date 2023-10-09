use super::service::*;
use actix_web::{ Responder, web};
use crate::services::dto::*;
use actix_web::HttpResponseBuilder;

pub async fn get_service(path: web::Path<GetOneService>) -> impl Responder {
    let my_service = get_one_service(path.id).await;
    // HttpResponse::Ok().json(my_service)
    HttpResponseBuilder::new(my_service.status).json(my_service)
}

pub async fn get() -> impl Responder {
    let my_services = get_services_service().await;
    HttpResponseBuilder::new(my_services.status).json(my_services)
}

pub async fn create(form: web::Json<UploadForm>) -> impl Responder {
    let my_service = create_service(form.into_inner()).await;
    HttpResponseBuilder::new(my_service.status).json(my_service)
}
