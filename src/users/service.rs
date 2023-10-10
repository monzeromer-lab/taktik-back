use super::dto::RegisterUserForm;
use crate::db::create_connection;
use crate::models::prelude::*;
use crate::models::{self, user};
use crate::shared::*;
use actix_web::http::StatusCode;
use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use chrono::Utc;
use sea_orm::ActiveValue::{NotSet, Set};
use sea_orm::ActiveModelTrait;

pub async fn register_user(data: RegisterUserForm) -> TakTikResponse<models::user::Model> {
    let db_conn = create_connection().await.unwrap();
    match User::find()
        .filter(user::Column::Email.eq(&data.email))
        .one(&db_conn)
        .await
    {
        Ok(user) => match user {
            None => {
                let now = Utc::now();
                let new_user = models::user::ActiveModel {
                    name: Set(data.name),
                    email: Set(data.email),
                    password: Set(data.password),
                    created_at: Set(now.to_string()),
                    updated_at: Set(now.to_string()),
                    id: NotSet,
                    name_ar: NotSet,
                    image: NotSet,
                    deleted_at: NotSet,
                    status: NotSet,
                    active: NotSet,
                };

                match new_user.insert(&db_conn).await {
                    Ok(user_result) => {
                        TakTikResponse {
                            error: false,
                            status: StatusCode::CREATED,
                            result: ResResult::Data(user_result)
                        }
                    },
                    Err(err) => {
                        println!("{:?}", err);
                        TakTikResponse {
                            error: true,
                            status: StatusCode::INTERNAL_SERVER_ERROR,
                            result: ResResult::Error(ResError::Error("Something went wrong".to_string()))
                        }
                    }
                }
                
            },
            Some(..) => TakTikResponse {
                error: true,
                status: StatusCode::BAD_REQUEST,
                result: ResResult::Error(
                    ResError::Error("User already exists, reset your password if you forgot it.".to_string()),
                ),
            },
        },
        Err(err) =>{ 
            println!("{:?}", err);
            TakTikResponse {
            error: true,
            status: StatusCode::INTERNAL_SERVER_ERROR,
            result: ResResult::Error(ResError::Error("Something went wrong".to_string())),
        }},
    }

}
