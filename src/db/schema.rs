// @generated automatically by Diesel CLI.
use diesel::*;

table! {
    artical (id) {
        id -> Integer,
        title -> Text,
        desc -> Text,
        image -> Text,
        post_type -> Text,
        created_at -> Text,
        updated_at -> Text,
        deleted_at -> Nullable<Text>,
        status -> Bool,
        creator -> Integer,
    }
}

table! {
    service (id) {
        id -> Integer,
        title -> Text,
        desc -> Text,
        image -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Timestamp,
        status -> Bool,
        creator -> Integer,
    }
}

table! {
    user (id) {
        id -> Integer,
        name -> Text,
        name_ar -> Text,
        email -> Text,
        password -> Text,
        image -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Timestamp,
        status -> Bool,
        active -> Bool,
    }
}

joinable!(artical -> user (creator));
joinable!(service -> user (creator));

allow_tables_to_appear_in_same_query!(
    artical,
    service,
    user,
);
