pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenvy::dotenv;
use models::{NewTask, Task, UpdateTask};
use std::env;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_task(
    conn: &mut SqliteConnection,
    title: &str,
    description: Option<&str>,
    status: Option<&str>,
) -> Task {
    use schema::tasks::dsl::tasks;
    let new_task = NewTask {
        title,
        description,
        status: status.unwrap_or("pending"),
    };
    diesel::insert_into(tasks)
        .values(&new_task)
        .returning(Task::as_returning())
        .get_result(conn)
        .expect("Error inserting new task")
}

pub fn update_task(
    conn: &mut SqliteConnection,
    id: i32,
    title: Option<&str>,
    description: Option<&str>,
    status: Option<&str>,
) -> Task {
    let update_task = UpdateTask {
        id,
        title,
        description,
        status,
    };
    update_task.save_changes(conn).expect("Error updating task")
}

pub fn delete_task(conn: &mut SqliteConnection, task_id: i32) -> usize {
    use schema::tasks::dsl::{id, tasks};
    diesel::delete(tasks)
        .filter(id.eq(&task_id))
        .execute(conn)
        .expect("Error deleting tasks")
}
