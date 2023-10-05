mod guards;
mod models;
mod services;
mod shared;
mod db;

use crate::articales::articales_module;
use crate::catalog::catalogs_module;
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
