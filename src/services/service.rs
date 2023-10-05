use crate::shared::crud::CRUD;
use crate::db::create_connection;
use crate::models::prelude::*;
use sea_orm::DatabaseConnection;

pub async fn get_services_service(){
    let db_conn = create_connection().await;
    let my_data = CRUD::read::<DatabaseConnection, Service, ()>(None, Some(db_conn.unwrap()));

    my_data.await
}