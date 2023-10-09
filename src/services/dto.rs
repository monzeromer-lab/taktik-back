use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct UploadForm {
    #[validate(length(min = 1))]
    pub title: String,

    #[validate(length(min = 1))]
    pub desc: String,

    #[validate(range(min = 1))]
    pub creator: i32,
}

#[derive(Debug, Serialize)]
pub struct NewServiceResult{
    pub id: i32
}

#[derive(Debug, Deserialize, Validate)]
pub struct GetOneService {
    #[validate(range(min = 1))]
    pub id: i32,
}
