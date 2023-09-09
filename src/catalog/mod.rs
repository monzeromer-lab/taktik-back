mod controller;

use actix_web::{dev::HttpServiceFactory, services, web};
use controller::get_catalog;

pub fn catalogs_module() -> impl HttpServiceFactory {
    let services = services![
        web::resource("/catalog").to(|| get_catalog()),
    ];
    services
}
