use actix_web::{Responder, HttpResponse};
use super::service::get_services_service;

pub async fn get_service() -> impl Responder {
    println!("services route");
    HttpResponse::Ok().body("services!")
}

pub async fn get_services() -> impl Responder {
    let my_services = get_services_service().await;

    HttpResponse::Ok().json(my_services.unwrap())
}
