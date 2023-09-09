mod controller;

use actix_web::{dev::HttpServiceFactory, services, web};
use controller::get_service;

pub fn services_module() -> impl HttpServiceFactory {
    let services = services![
        web::resource("/service").to(|| get_service()),
    ];
    services
}
