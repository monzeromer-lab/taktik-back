use diesel::prelude::*;
use diesel::QueryableByName;
use serde::{Deserialize, Serialize};
use crate::db::schema::artical;

#[derive(Debug, Queryable, QueryableByName, PartialEq, Serialize, Deserialize)]
#[diesel(table_name = artical)]
pub struct Artical {
    pub id: i32,
    pub title: String,
    pub desc: String,
    pub image: String,
    pub post_type: String,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
    pub status: bool,
    pub creator: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = artical)]
pub struct NewArtical<'a> {
    pub title: &'a str,
    pub desc: &'a str,
    pub image: &'a str,
    pub post_type: &'a str,
    pub created_at: &'a str,
    pub updated_at: &'a str,
    pub deleted_at: Option<&'a str>,
    pub status: bool,
    pub creator: i32,
}