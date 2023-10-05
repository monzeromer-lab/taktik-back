use diesel::prelude::*;
use diesel::QueryableByName;
use serde::{Deserialize, Serialize};
use crate::db::schema::service;

#[derive(Debug, Queryable, QueryableByName, PartialEq, Serialize, Deserialize)]
#[diesel(table_name = service)]
pub struct Service {
    pub id: i32,
    pub title: String,
    pub desc: String,
    pub image: String,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
    pub status: bool,
    pub creator: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = service)]
pub struct NewService<'a>{
    pub title: &'a str,
    pub desc: &'a str,
    pub image: &'a str,
    pub created_at: &'a str,
    pub updated_at: &'a str,
    pub deleted_at: Option<String>,
    pub status: bool,
    pub creator: i32,
}