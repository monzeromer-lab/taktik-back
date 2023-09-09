mod controller;

use actix_web::{dev::HttpServiceFactory, services, web};
use controller::{get_user, hello_world};

pub fn articales_module() -> impl HttpServiceFactory {
    let services = services![
        web::resource("/artcale").to(|| get_artical()),
    ];
    services
}
