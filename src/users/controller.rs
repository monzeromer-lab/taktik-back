use actix_web::{ Responder, web, http::StatusCode};
use crate::shared::{TakTikResponse, ResResult, ResError};
use super::dto::RegisterUserForm;
use super::service::register_user;
use actix_web::HttpResponseBuilder;
use validator::Validate;

pub async fn register_new_user(form: web::Json<RegisterUserForm>) -> impl Responder {
    match form.validate() {
        Ok(_) => {
            let my_new_user = register_user(form.into_inner()).await;
            HttpResponseBuilder::new(my_new_user.status).json(my_new_user)
        },
        Err(err) => HttpResponseBuilder::new(StatusCode::BAD_REQUEST).json(TakTikResponse {
            error: true,
            status: StatusCode::BAD_REQUEST,
            result: ResResult::<()>::Error(ResError::ValidationErrors(err))
        })
            
    }
}