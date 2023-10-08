use crate::db::create_connection;
use crate::models;
use crate::models::prelude::*;
use sea_orm::EntityTrait;
use sea_orm::ActiveValue::{Set, NotSet};
use super::dto::*;
use chrono::Utc;



pub async fn get_services_service() -> Vec<models::service::Model> {
    let db_conn = create_connection().await.unwrap();

    let my_services = Service::find()
    .all(&db_conn)
    .await.unwrap();

    my_services
}

pub async fn create_service(data: UploadForm) -> NewServiceResult{
    let db_conn = create_connection().await.unwrap();

    let now = Utc::now();

    let new_service = models::service::ActiveModel {
        id: NotSet,
        title: Set(Some(data.title)),
        desc: Set(Some(data.desc)),
        created_at: Set(Some(now.to_string())),
        updated_at: Set(Some(now.to_string())),
        status: Set(true),
        creator: Set(Some(data.creator)),
        image: Set(None),
        deleted_at: Set(None),
    };
    
    
    let pear: sea_orm::InsertResult<models::service::ActiveModel> = Service::insert(new_service).exec(&db_conn).await.unwrap();

    NewServiceResult{
        id: pear.last_insert_id
    }
}
