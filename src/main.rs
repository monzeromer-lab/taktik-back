use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
mod database;
mod guards;
mod models;
mod routers;
mod services;
mod shared;


#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Taktik server is running at: http://127.0.0.1:8080");
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(crate::routers::users_module())
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
