mod controller;
mod dto;
mod service;

use actix_web::{dev::HttpServiceFactory, services, web};
use controller::{get_user, hello_world};

pub fn users_module() -> impl HttpServiceFactory {
    let services = services![
        web::resource("/test").to(|| get_user()),
        web::scope("/hi").route("/", web::get().to(|| hello_world()))
    ];
    services
}
