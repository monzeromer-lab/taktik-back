// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Text,
        title -> Text,
        body -> Text,
        published -> Bool,
    }
}
