use crate::schema;
use crate::schema::tasks;
use diesel::prelude::*;
use diesel::result::Error;

pub const DEFAULT_TASK_TITLE: &str = "Untitled Task";
pub const DEFAULT_TASK_STATUS: &str = "pending";

#[derive(Debug, Queryable, Selectable)]
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

impl Task {
    pub fn find(conn: &mut SqliteConnection, id: i32) -> Result<Task, Error> {
        use schema::tasks::dsl::tasks;
        tasks.find(id).first(conn)
    }
    pub fn list(conn: &mut SqliteConnection) -> Result<Vec<Task>, Error> {
        use schema::tasks::dsl::tasks;
        tasks.load::<Task>(conn)
    }
    pub fn create(
        conn: &mut SqliteConnection,
        title: &str,
        description: Option<&str>,
        status: Option<&str>,
    ) -> Result<Task, Error> {
        use schema::tasks::dsl::tasks;
        let effective_title = if title.trim().is_empty() {
            DEFAULT_TASK_TITLE
        } else {
            title
        };
        let new_task = NewTask {
            title: effective_title,
            description,
            status: status.unwrap_or(DEFAULT_TASK_STATUS),
        };
        diesel::insert_into(tasks)
            .values(&new_task)
            .returning(Task::as_returning())
            .get_result(conn)
    }

    pub fn update(
        conn: &mut SqliteConnection,
        id: i32,
        title: Option<&str>,
        description: Option<&str>,
        status: Option<&str>,
    ) -> Result<Task, Error> {
        let update_task = UpdateTask {
            id,
            title,
            description,
            status,
        };
        update_task.save_changes(conn)
    }

    pub fn delete(conn: &mut SqliteConnection, task_id: i32) -> Result<usize, Error> {
        use schema::tasks::dsl::{id, tasks};
        diesel::delete(tasks).filter(id.eq(&task_id)).execute(conn)
    }
}
