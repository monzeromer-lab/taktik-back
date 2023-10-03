use diesel::prelude::Queryable;
use std::time::SystemTime;

#[derive(Queryable, PartialEq, Debug)]
pub struct Service {
    id: i32,
    title: String,
    desc: String,
    image: String,
    created_at: SystemTime,
    updated_at: SystemTime,
    deleted_at: SystemTime,
}
