mod guards;
mod models;
mod users;
mod articales;
mod services;
mod site;
mod catalog;
mod shared;
mod db;

use actix_web::{App, HttpServer};
use crate::users::users_module;
use crate::articales::articales_module;
use crate::services::services_module;
use crate::site::sites_module;
use crate::catalog::catalogs_module;
use crate::db::create_connection;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    let db = create_connection().await;

    println!("Taktik server is running at: http://127.0.0.1:8080");
    HttpServer::new(|| {

        App::new()
            .service(users_module())
            .service(articales_module())
            .service(services_module())
            .service(sites_module())
            .service(catalogs_module())

    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
