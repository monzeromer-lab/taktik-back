use actix_web::{HttpResponse, Responder};

pub async fn get_service() -> impl Responder {
    println!("services route");
    HttpResponse::Ok().body("services!")
}
