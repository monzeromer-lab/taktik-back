use actix_web::{Responder, HttpResponse};

pub async fn get_service() -> impl Responder {
    println!("services route");
    HttpResponse::Ok().body("services!")
}