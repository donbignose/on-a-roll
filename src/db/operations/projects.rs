use crate::models::{NewProject, Project, UpdateProject};
use crate::schema::projects::dsl::projects;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::SqliteConnection;
impl Project {
    pub fn find(conn: &mut SqliteConnection, id: i32) -> Result<Self, Error> {
        projects.find(id).first(conn)
    }
    pub fn list(conn: &mut SqliteConnection) -> Result<Vec<Self>, Error> {
        projects.load::<Self>(conn)
    }
    pub fn create(
        conn: &mut SqliteConnection,
        title: Option<&str>,
        description: Option<&str>,
        status: Option<&str>,
    ) -> Result<Self, Error> {
        let new_project = NewProject {
            title,
            description,
            status,
        };
        diesel::insert_into(projects)
            .values(&new_project)
            .returning(Self::as_returning())
            .get_result(conn)
    }

    pub fn update(
        conn: &mut SqliteConnection,
        id: i32,
        title: Option<&str>,
        description: Option<&str>,
        status: Option<&str>,
    ) -> Result<Self, Error> {
        let update_project = UpdateProject {
            id,
            title,
            description,
            status,
        };
        update_project.save_changes(conn)
    }

    pub fn delete(conn: &mut SqliteConnection, project_id: i32) -> Result<usize, Error> {
        use crate::schema::projects::id;
        diesel::delete(projects)
            .filter(id.eq(&project_id))
            .execute(conn)
    }
}
