use super::projects::Project;
use crate::schema::tasks;
use diesel::prelude::*;

pub const DEFAULT_TASK_TITLE: &str = "New Task";
pub const DEFAULT_TASK_STATUS: &str = "Todo";

#[derive(Debug, Queryable, Selectable, Associations)]
#[diesel(belongs_to(Project))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub project_id: Option<i32>,
}

#[derive(Insertable)]
#[diesel(table_name = tasks)]
pub struct NewTask<'a> {
    pub title: Option<&'a str>,
    pub description: Option<&'a str>,
    pub status: Option<&'a str>,
    pub project_id: Option<i32>,
}

#[derive(AsChangeset, Identifiable)]
#[diesel(table_name = tasks)]
pub struct UpdateTask<'a> {
    pub id: i32,
    pub title: Option<&'a str>,
    pub description: Option<&'a str>,
    pub status: Option<&'a str>,
    pub project_id: Option<i32>,
}
