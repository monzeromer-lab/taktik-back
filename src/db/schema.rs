// @generated automatically by Diesel CLI.

diesel::table! {
    artical (id) {
        id -> Integer,
        title -> Text,
        desc -> Text,
        image -> Text,
        post_type -> Text,
        createdAt -> Timestamp,
        updatedAt -> Timestamp,
        deletedAt -> Timestamp,
        status -> Bool,
        creator -> Integer,
    }
}

diesel::table! {
    service (id) {
        id -> Integer,
        title -> Text,
        desc -> Text,
        image -> Text,
        createdAt -> Timestamp,
        updatedAt -> Timestamp,
        deletedAt -> Timestamp,
        status -> Bool,
        creator -> Integer,
    }
}

diesel::table! {
    user (id) {
        id -> Integer,
        name -> Text,
        name_ar -> Text,
        email -> Text,
        password -> Text,
        image -> Text,
        createdAt -> Timestamp,
        updatedAt -> Timestamp,
        deletedAt -> Timestamp,
        status -> Bool,
        active -> Bool,
    }
}

diesel::joinable!(artical -> user (creator));
diesel::joinable!(service -> user (creator));

diesel::allow_tables_to_appear_in_same_query!(
    artical,
    service,
    user,
);
