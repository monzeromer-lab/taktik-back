mod controller;

use actix_web::{dev::HttpServiceFactory, services, web};
use controller::get_artical;

pub fn articales_module() -> impl HttpServiceFactory {
    services![web::resource("/artcale").to(get_artical)]
}

