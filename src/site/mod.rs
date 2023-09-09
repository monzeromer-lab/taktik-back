mod controller;

use actix_web::{dev::HttpServiceFactory, services, web};
use controller::get_site;

pub fn sites_module() -> impl HttpServiceFactory {
    let services = services![
        web::resource("/site").to(|| get_site()),
    ];
    services
}
