// @generated automatically by Diesel CLI.

diesel::table! {
    use crate::models::project_status::ProjectStatusMapping;
    use diesel::sql_types::{Integer, Text, Nullable};
    projects (id) {
        id -> Integer,
        title -> Text,
        description -> Nullable<Text>,
        status -> ProjectStatusMapping,
    }
}

diesel::table! {
    use crate::models::task_status::TaskStatusMapping;
    use diesel::sql_types::{Integer, Text, Nullable};
    tasks (id) {
        id -> Integer,
        title -> Text,
        description -> Nullable<Text>,
        status -> TaskStatusMapping,
        project_id -> Nullable<Integer>,
    }
}

diesel::joinable!(tasks -> projects (project_id));

diesel::allow_tables_to_appear_in_same_query!(projects, tasks,);
