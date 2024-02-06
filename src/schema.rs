// @generated automatically by Diesel CLI.

diesel::table! {
    tasks (id) {
        id -> Integer,
        title -> Text,
        description -> Nullable<Text>,
        status -> Text,
    }
}
