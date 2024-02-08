// @generated automatically by Diesel CLI.

diesel::table! {
    projects (id) {
        id -> Integer,
        title -> Text,
        description -> Nullable<Text>,
        status -> Text,
    }
}

diesel::table! {
    tasks (id) {
        id -> Integer,
        title -> Text,
        description -> Nullable<Text>,
        status -> Text,
        project_id -> Nullable<Integer>,
    }
}

diesel::joinable!(tasks -> projects (project_id));

diesel::allow_tables_to_appear_in_same_query!(projects, tasks,);
