use diesel::prelude::Queryable;
use std::time::SystemTime;

#[derive(Queryable, PartialEq, Debug)]
pub struct User {
    id: i32,
    name: String,
    email: String,
    password: String,
    image: String,
    created_at: SystemTime,
    updated_at: SystemTime,
    deleted_at: SystemTime,
}
