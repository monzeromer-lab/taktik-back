use actix_web::{HttpResponse, Responder};

pub async fn get_site() -> impl Responder {
    println!("site route");
    HttpResponse::Ok().body("site!")
}
