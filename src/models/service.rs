//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.3

use serde::Serialize;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize)]
#[sea_orm(table_name = "service")]
pub struct Model {
    #[sea_orm(primary_key)]
    id: i32,
    #[sea_orm(nullable)]
    pub title: Option<String>,
    #[sea_orm(nullable)]
    pub desc: Option<String>,
    #[sea_orm(nullable)]
    pub image: Option<String>,
    #[sea_orm(
        column_name = "createdAt",
        nullable
    )]
    pub created_at: Option<String>,
    #[sea_orm(
        column_name = "updatedAt",
        nullable
    )]
    pub updated_at: Option<String>,
    #[sea_orm(
        column_name = "deletedAt",
        nullable
    )]
    pub deleted_at: Option<String>,
    pub status: bool,
    #[sea_orm(nullable)]
    pub creator: Option<u64>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, Serialize)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::Creator",
        to = "super::user::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    User,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
