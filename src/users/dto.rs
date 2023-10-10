use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterUserForm {
    #[validate(length(min = 1))]
    pub name: String,

    #[validate(email)]
    pub email: String,

    #[validate(length(min = 1))]
    pub password: String,
}