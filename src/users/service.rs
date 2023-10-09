use super::dto::RegisterUserForm;
use crate::db::create_connection;
use crate::models;
use crate::models::prelude::*;
use crate::shared::TakTikResponse;

pub async fn register_user(data: RegisterUserForm) {
    let db_conn = create_connection().await.unwrap();

    todo!()
}