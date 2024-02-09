use crate::schema::projects;
use diesel::prelude::*;

pub const DEFAULT_PROJECT_TITLE: &str = "New Project";
pub const DEFAULT_PROJECT_STATUS: &str = "Planning";
#[derive(Debug, Queryable, Selectable)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Project {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
}
#[derive(Insertable)]
#[diesel(table_name = projects)]
pub struct NewProject<'a> {
    pub title: Option<&'a str>,
    pub description: Option<&'a str>,
    pub status: Option<&'a str>,
}
#[derive(AsChangeset, Identifiable)]
#[diesel(table_name = projects)]
pub struct UpdateProject<'a> {
    pub id: i32,
    pub title: Option<&'a str>,
    pub description: Option<&'a str>,
    pub status: Option<&'a str>,
}
