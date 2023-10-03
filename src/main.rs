mod articales;
mod catalog;
mod database;
mod guards;
mod models;
mod services;
mod shared;
mod site;
mod users;

use crate::articales::articales_module;
use crate::catalog::catalogs_module;
use crate::services::services_module;
use crate::site::sites_module;
use crate::users::users_module;
use actix_files::Files;
use actix_web::{App, HttpServer};

const PORT: u16 = 3985;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Taktik server is running at: http://127.0.0.1:3985");
    HttpServer::new(|| {
        App::new()
            .service(Files::new("/static", ".").show_files_listing())
            .service(users_module())
            .service(articales_module())
            .service(services_module())
            .service(sites_module())
            .service(catalogs_module())
    })
    .bind(("0.0.0.0", PORT))?
    .run()
    .await
}
