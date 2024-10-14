use super::projects::Project;
use super::task_status::TaskStatus;
use crate::schema::tasks;
use diesel::prelude::*;
use ratatui::widgets::ListItem;

pub const DEFAULT_TASK_TITLE: &str = "New Task";
pub const DEFAULT_TASK_STATUS: TaskStatus = TaskStatus::Todo;

#[derive(Debug, Queryable, Selectable, Associations)]
#[diesel(belongs_to(Project))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub status: TaskStatus,
    pub project_id: Option<i32>,
}

impl<'a> From<&'a Task> for ListItem<'a> {
    fn from(task: &'a Task) -> Self {
        ListItem::new(format!("{}: {}", task.id, task.title))
    }
}

#[derive(Insertable)]
#[diesel(table_name = tasks)]
pub struct NewTask<'a> {
    pub title: Option<&'a str>,
    pub description: Option<&'a str>,
    pub status: Option<TaskStatus>,
    pub project_id: Option<i32>,
}

#[derive(AsChangeset, Identifiable)]
#[diesel(table_name = tasks)]
pub struct UpdateTask<'a> {
    pub id: i32,
    pub title: Option<&'a str>,
    pub description: Option<&'a str>,
    pub status: Option<TaskStatus>,
    pub project_id: Option<i32>,
}
