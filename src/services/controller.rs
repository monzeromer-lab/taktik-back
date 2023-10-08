use super::service::*;
use actix_web::{ HttpResponse, Responder, web};
use crate::services::dto::UploadForm;

pub async fn get_service() -> impl Responder {
    println!("services route");
    HttpResponse::Ok().body("services!")
}

pub async fn get() -> impl Responder {
    let my_services = get_services_service().await;
    HttpResponse::Ok().json(my_services)
}

pub async fn create(form: web::Json<UploadForm>) -> impl Responder {
    let my_service = create_service(form.into_inner()).await;
    HttpResponse::Ok().json(my_service)
}
