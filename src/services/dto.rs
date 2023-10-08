use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;
// MultipartForm,
#[derive(Debug, Deserialize, Validate)]
pub struct UploadForm {
    #[validate(length(min = 1))]
    pub title: String,

    #[validate(length(min = 1))]
    pub desc: String,

    #[validate(length(min = 1))]
    pub image_name: String,

    pub created_at: DateTime<Utc>,

    pub updated_at: DateTime<Utc>,

    pub deleted_at: DateTime<Utc>,

    pub status: bool,

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
