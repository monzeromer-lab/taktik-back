use actix_web::{HttpResponse, Responder};

pub async fn get_catalog() -> impl Responder {
    println!("catalog route");
    HttpResponse::Ok().body("catalog!")
}
