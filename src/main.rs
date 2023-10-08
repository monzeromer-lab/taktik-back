mod guards;
mod models;
mod services;
mod articales;
mod site;
mod catalog;
mod users;
mod shared;
mod db;

use actix_files::Files;
use actix_web::{App, HttpServer};
use crate::articales::articales_module;
use crate::catalog::catalogs_module;
use crate::services::services_module;
use crate::site::sites_module;
use crate::users::users_module;
use actix_multipart::form::tempfile::TempFileConfig;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    std::fs::create_dir_all("./tmp")?;
    println!("Taktik server is running at: http://127.0.0.1:8080");
    HttpServer::new(|| {
        App::new()
            .app_data(TempFileConfig::default().directory("./tmp"))
            .service(Files::new("/static", ".").show_files_listing())
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
