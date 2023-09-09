use actix_web::{Responder, HttpResponse};

pub async fn get_site() -> impl Responder {
    println!("site route");
    HttpResponse::Ok().body("site!")
}