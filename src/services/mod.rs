mod controller;
mod dto;
mod service;

use actix_web::{dev::HttpServiceFactory, services, web};
use controller::{create, get, get_service};

pub fn services_module() -> impl HttpServiceFactory {
    let services = services![
        web::scope("/service")
            .route("/", web::get().to(get))
            .route("/new", web::post().to(create)),
        web::resource("/find/{id}").route(web::get().to(get_service))
    ];
    services
}
