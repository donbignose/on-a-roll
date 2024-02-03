use crate::schema::tasks;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
}

#[derive(Insertable)]
#[diesel(table_name = tasks)]
pub struct NewTask<'a> {
    pub title: &'a str,
    pub description: Option<&'a str>,
    pub status: &'a str,
}

#[derive(AsChangeset, Identifiable)]
#[diesel(table_name = tasks)]
pub struct UpdateTask<'a> {
    pub id: i32,
    pub title: Option<&'a str>,
    pub description: Option<&'a str>,
    pub status: Option<&'a str>,
}
