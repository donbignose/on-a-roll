mod common;
use common::establish_test_connection;
use on_a_roll::models::{Project, DEFAULT_PROJECT_STATUS, DEFAULT_PROJECT_TITLE};
#[test]
fn test_create_project() {
    let mut conn = establish_test_connection();
    let project = Project::create(&mut conn, Some("Test Project"), None, Some("active")).unwrap();

    assert_eq!(project.title, "Test Project");
    assert_eq!(project.status, "active");
}
#[test]
fn test_create_default_project() {
    let mut conn = establish_test_connection();
    let project = Project::create(&mut conn, None, None, None).unwrap();

    assert_eq!(project.title, DEFAULT_PROJECT_TITLE);
    assert_eq!(project.status, DEFAULT_PROJECT_STATUS);
}
#[test]
fn test_update_project() {
    let mut conn = establish_test_connection();
    let project = Project::create(&mut conn, Some("Update Project"), None, Some("active")).unwrap();
    let updated_project =
        Project::update(&mut conn, project.id, Some("Updated Title"), None, None).unwrap();

    assert_eq!(updated_project.title, "Updated Title");
}
#[test]
fn test_delete_project() {
    let mut conn = establish_test_connection();
    let project = Project::create(&mut conn, Some("Delete Project"), None, Some("active")).unwrap();

    let num_deleted = Project::delete(&mut conn, project.id).unwrap();
    assert_eq!(num_deleted, 1);
}
#[test]
fn test_find_project() {
    let mut conn = establish_test_connection();

    let project = Project::create(&mut conn, Some("Test Project"), None, Some("active")).unwrap();

    let found_project = Project::find(&mut conn, project.id).unwrap();
    assert_eq!(found_project.title, "Test Project");
    assert_eq!(found_project.status, "active");
}
#[test]
fn test_list_projects() {
    let mut conn = establish_test_connection();

    Project::create(&mut conn, Some("Project 1"), None, Some("active")).unwrap();
    Project::create(&mut conn, Some("Project 2"), None, Some("active")).unwrap();

    let projects = Project::list(&mut conn).unwrap();
    assert_eq!(projects.len(), 2);
}
