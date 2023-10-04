use diesel::prelude::*;
use diesel::sql_types::*;
use diesel::QueryableByName;

#[derive(Debug, Queryable, QueryableByName)]
pub struct Artical {
    pub id: i32,
    pub title: String,
    pub desc: String,
    pub image: String,
    pub post_type: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
    pub status: bool,
    pub creator: i32,
}

#[derive(Insertable)]
#[table_name = "artical"]
pub struct NewPost<'a> {
    pub id: i32,
    pub title: &'a str,
    pub desc: &'a str,
    pub image: &'a str,
    pub post_type: &'a str,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
    pub deleted_at: Option<SystemTime>,
    pub status: Bool,
    pub creator: i32,
}