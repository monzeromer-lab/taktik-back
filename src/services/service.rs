use crate::shared::crud::CRUD;
use crate::db::create_connection;
use crate::models;
use crate::models::prelude::*;
use sea_orm::{DatabaseConnection, Condition};

// pub async fn get_services_service(){
//     let db_conn = create_connection().await;
//     let my_data = CRUD::read::<DatabaseConnection, Service, ()>(None, Some(db_conn.unwrap()));

//     my_data.await
// }

// pub async fn get_services_service() -> Result<Vec<models::service::Model>, sea_orm::DbErr> {
//     let db_conn: Result<DatabaseConnection, sea_orm::DbErr> = create_connection().await;
//     let my_data = CRUD::read::<DatabaseConnection, Service, Condition>(Some(Condition::any()), Some(&db_conn.unwrap()));

//     my_data.await
// }

pub async fn get_services_service() -> Result<Vec<models::service::Model>, sea_orm::DbErr> {
    let db_conn = create_connection().await.unwrap();
    let my_data = CRUD::read::<DatabaseConnection, Service, Condition>(
        Some(Condition::any()),
        Some(&db_conn),
    );

    let result = my_data.await;
    // Process the result further...
    result
}
