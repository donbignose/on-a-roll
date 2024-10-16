use super::project_status::ProjectStatus;
use crate::schema::projects;
use diesel::prelude::*;
use ratatui::widgets::ListItem;

pub const DEFAULT_PROJECT_TITLE: &str = "New Project";
pub const DEFAULT_PROJECT_STATUS: ProjectStatus = ProjectStatus::Planning;
#[derive(Debug, Clone, Queryable, Selectable, PartialEq)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Project {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub status: ProjectStatus,
}

impl<'a> From<Project> for ListItem<'a> {
    fn from(project: Project) -> Self {
        ListItem::new(format!("{}: {}", project.id, project.title))
    }
}

#[derive(Debug, Insertable)]
#[diesel(table_name = projects)]
pub struct NewProject<'a> {
    pub title: Option<&'a str>,
    pub description: Option<&'a str>,
    pub status: Option<ProjectStatus>,
}
#[derive(Debug, AsChangeset, Identifiable)]
#[diesel(table_name = projects)]
pub struct UpdateProject<'a> {
    pub id: i32,
    pub title: Option<&'a str>,
    pub description: Option<&'a str>,
    pub status: Option<ProjectStatus>,
}
