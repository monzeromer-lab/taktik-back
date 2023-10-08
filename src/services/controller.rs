use super::service::*;
use actix_web::{ HttpResponse, Responder, web};
use crate::services::dto::*;

pub async fn get_service(path: web::Path<GetOneService>) -> impl Responder {
    let my_service = get_one_service(path.id).await;
    HttpResponse::Ok().json(my_service)
}

pub async fn get() -> impl Responder {
    let my_services = get_services_service().await;
    HttpResponse::Ok().json(my_services)
}

pub async fn create(form: web::Json<UploadForm>) -> impl Responder {
    let my_service = create_service(form.into_inner()).await;
    HttpResponse::Ok().json(my_service)
}
