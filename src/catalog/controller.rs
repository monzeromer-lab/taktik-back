use actix_web::{Responder, HttpResponse};

pub async fn get_catalog() -> impl Responder {
    println!("catalog route");
    HttpResponse::Ok().body("catalog!")
}