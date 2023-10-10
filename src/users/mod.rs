mod controller;
mod dto;
mod service;

use actix_web::{dev::HttpServiceFactory, services, web};
use controller::register_new_user;

pub fn users_module() -> impl HttpServiceFactory {
    let services = services![
        web::resource("/signup").route(web::post().to(register_new_user))
    ];
    services
}
