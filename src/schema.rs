// @generated automatically by Diesel CLI.

diesel::table! {
    task (id) {
        id -> Integer,
        title -> Text,
        description -> Nullable<Text>,
        status -> Text,
    }
}