use crate::schema;
use crate::schema::{projects, tasks};
use diesel::prelude::*;
use diesel::result::Error;

pub const DEFAULT_TASK_TITLE: &str = "New Task";
pub const DEFAULT_TASK_STATUS: &str = "Todo";
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

impl Project {
    pub fn find(conn: &mut SqliteConnection, id: i32) -> Result<Project, Error> {
        use schema::projects::dsl::projects;
        projects.find(id).first(conn)
    }
    pub fn list(conn: &mut SqliteConnection) -> Result<Vec<Project>, Error> {
        use schema::projects::dsl::projects;
        projects.load::<Project>(conn)
    }
    pub fn create(
        conn: &mut SqliteConnection,
        title: Option<&str>,
        description: Option<&str>,
        status: Option<&str>,
    ) -> Result<Project, Error> {
        use schema::projects::dsl::projects;
        let new_project = NewProject {
            title,
            description,
            status,
        };
        diesel::insert_into(projects)
            .values(&new_project)
            .returning(Project::as_returning())
            .get_result(conn)
    }

    pub fn update(
        conn: &mut SqliteConnection,
        id: i32,
        title: Option<&str>,
        description: Option<&str>,
        status: Option<&str>,
    ) -> Result<Project, Error> {
        let update_project = UpdateProject {
            id,
            title,
            description,
            status,
        };
        update_project.save_changes(conn)
    }

    pub fn delete(conn: &mut SqliteConnection, project_id: i32) -> Result<usize, Error> {
        use schema::projects::dsl::{id, projects};
        diesel::delete(projects)
            .filter(id.eq(&project_id))
            .execute(conn)
    }
}
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
        title: Option<&str>,
        description: Option<&str>,
        status: Option<&str>,
        project_id: Option<i32>,
    ) -> Result<Task, Error> {
        use schema::tasks::dsl::tasks;
        let new_task = NewTask {
            title,
            description,
            status,
            project_id,
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
        project_id: Option<i32>,
    ) -> Result<Task, Error> {
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
        use schema::tasks::dsl::{id, tasks};
        diesel::delete(tasks).filter(id.eq(&task_id)).execute(conn)
    }
}
