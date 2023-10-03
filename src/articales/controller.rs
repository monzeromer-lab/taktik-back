use actix_web::{HttpResponse, Responder};

pub async fn get_artical() -> impl Responder {
    println!("artical route");
    HttpResponse::Ok().body("artical!")
}
