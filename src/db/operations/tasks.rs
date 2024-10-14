use crate::models::task_status::TaskStatus;
use crate::models::{NewTask, Task, UpdateTask};
use crate::schema::tasks::dsl::tasks;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::SqliteConnection;
impl Task {
    pub fn find(conn: &mut SqliteConnection, id: i32) -> Result<Self, Error> {
        tasks.find(id).first(conn)
    }
    pub fn list(conn: &mut SqliteConnection) -> Result<Vec<Self>, Error> {
        tasks.load::<Self>(conn)
    }
    pub fn create(
        conn: &mut SqliteConnection,
        title: Option<&str>,
        description: Option<&str>,
        status: Option<TaskStatus>,
        project_id: Option<i32>,
    ) -> Result<Self, Error> {
        let new_task = NewTask {
            title,
            description,
            status,
            project_id,
        };
        diesel::insert_into(tasks)
            .values(&new_task)
            .returning(Self::as_returning())
            .get_result(conn)
    }

    pub fn update(
        conn: &mut SqliteConnection,
        id: i32,
        title: Option<&str>,
        description: Option<&str>,
        status: Option<TaskStatus>,
        project_id: Option<i32>,
    ) -> Result<Self, Error> {
        let update_task = UpdateTask {
            id,
            title,
            description,
            status,
            project_id,
        };
        update_task.save_changes(conn)
    }

    pub fn delete(conn: &mut SqliteConnection, task_id: i32) -> Result<usize, Error> {
        use crate::schema::tasks::dsl::id;
        diesel::delete(tasks).filter(id.eq(&task_id)).execute(conn)
    }
}
