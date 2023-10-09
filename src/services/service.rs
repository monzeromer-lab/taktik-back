use super::dto::*;
use crate::db::create_connection;
use crate::models;
use crate::models::prelude::*;
use crate::shared::TakTikResponse;
use crate::shared::*;
use chrono::Utc;
use sea_orm::ActiveValue::{NotSet, Set};
use sea_orm::EntityTrait;
use actix_web::http::StatusCode;

pub async fn get_services_service() -> TakTikResponse<Vec<models::service::Model>> {
    let db_conn = create_connection().await.unwrap();

    match Service::find().all(&db_conn).await {
        Ok(services) => TakTikResponse {
            error: false,
            status:  StatusCode::OK,
            result: ResResult::Data(services),
        },
        Err(err) => {
            println!("{:?}",err);
         TakTikResponse {
            error: true,
            status:  StatusCode::INTERNAL_SERVER_ERROR,
            result: ResResult::Error("Something went wrong".to_string()),
        }
    },
    }
}

pub async fn get_one_service(req_data: i32) -> TakTikResponse<models::service::Model> {
    let db_conn = create_connection().await.unwrap();
    match Service::find_by_id(req_data).one(&db_conn).await {
        Ok(my_service) => {
            match my_service {
                None => TakTikResponse {
                    error: true,
                    status: StatusCode::NOT_FOUND,
                    result: ResResult::Error("Service Not Found".to_string()),
                },
                Some(..) => TakTikResponse {
                    error: false,
                    status: StatusCode::OK,
                    result: ResResult::Data(my_service.unwrap()),
                }
            }
            
    },
        Err(err) => {
            println!("{:?}",err);
            TakTikResponse {
            error: true,
            status: StatusCode::INTERNAL_SERVER_ERROR,
            result: ResResult::Error("Something went wrong".to_string()),
        }
    },
    }
}

pub async fn create_service(data: UploadForm) -> TakTikResponse<NewServiceResult> {
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

    match Service::insert(new_service).exec(&db_conn).await {
        Ok(pear) => TakTikResponse {
            error: false,
            status: StatusCode::CREATED,
            result: ResResult::Data(NewServiceResult {
                id: pear.last_insert_id,
            }),
        },
        Err(err) => {
            println!("{:?}",err);
            TakTikResponse {
            error: true,
            status: StatusCode::INTERNAL_SERVER_ERROR,
            result: ResResult::Error("Something went wrong".to_string()),
        }
    },
    }
}
