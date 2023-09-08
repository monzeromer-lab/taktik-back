use actix_web::{dev::HttpServiceFactory, services, web};

pub fn users_module() -> impl HttpServiceFactory {
    let services = services![
        web::resource("/test").to(|| async { "test2" }),
        web::scope("/test3").route("/", web::get().to(|| async { "test3" }))
    ];
    services
}
