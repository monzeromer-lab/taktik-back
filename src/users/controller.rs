use actix_web::{HttpResponse, Responder};

pub async fn get_user() -> impl Responder {
    println!("users route");
    HttpResponse::Ok().body("users")
}

pub async fn hello_world() -> impl Responder {
    println!("hello world route");
    HttpResponse::Ok().body("Hello, World!")
}
