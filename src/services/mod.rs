mod controller;
mod service;
mod dto;

use actix_web::{dev::HttpServiceFactory, services, web};
use controller::{
    get_service,
    get,
    create,
};



pub fn services_module() -> impl HttpServiceFactory {
    let services = services![
        
        web::resource("/service").to(|| get_service()),
        web::resource("/services").to(|| get()),
    ];
    services
}
