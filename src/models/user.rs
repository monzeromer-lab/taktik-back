use diesel::prelude::*;
use diesel::QueryableByName;
use serde::{Deserialize, Serialize};
use crate::db::schema::user;

#[derive(Debug, Queryable, QueryableByName, PartialEq, Serialize, Deserialize)]
#[diesel(table_name = user)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub name_ar: String,
    pub email: String,
    pub password: String,
    pub image: String,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
    pub status: bool,
    pub active: bool,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = user)]
pub struct NewUser<'a>{
    pub id: i32,
    pub name: &'a str,
    pub name_ar: &'a str,
    pub email: &'a str,
    pub password: &'a str,
    pub image: &'a str,
    pub created_at: &'a str,
    pub updated_at: &'a str,
    pub deleted_at: Option<&'a str>,
    pub status: bool,
    pub active: bool,
}