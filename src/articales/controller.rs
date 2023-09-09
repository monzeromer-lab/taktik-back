use actix_web::{Responder, HttpResponse};

pub async fn get_artical() -> impl Responder {
    println!("artical route");
    HttpResponse::Ok().body("artical!")
}