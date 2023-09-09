use actix_web::{Responder, HttpResponse};

pub async fn get_user() -> impl Responder {
    println!("users route");
    HttpResponse::Ok().body("users")
}

pub async fn get_artical() -> impl Responder {
    println!("artical route");
    HttpResponse::Ok().body("artical!")
}