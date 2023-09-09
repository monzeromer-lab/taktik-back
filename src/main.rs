mod database;
mod guards;
mod models;
mod users;
mod articales;
mod services;
mod shared;

use actix_web::{App, HttpServer};
use crate::users::users_module;
use crate::articales::articales_module;
use crate::services::services_module;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Taktik server is running at: http://127.0.0.1:8080");
    HttpServer::new(|| {
        App::new()
            .service(users_module())
            .service(articales_module())
            .service(services_module())
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
