use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterUserForm {
    #[validate(length(min = 1), required)]
    pub name: Option<String>,

    #[validate(email, required)]
    pub email: Option<String>,

    #[validate(length(min = 1))]
    pub password: Option<String>,
}