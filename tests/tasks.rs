mod common;
use common::establish_test_connection;
use on_a_roll::{create_task, delete_task, update_task};

#[test]
fn test_create_task() {
    let mut conn = establish_test_connection();
    let task = create_task(&mut conn, "Test Task", Some("Description"), None);

    assert_eq!(task.title, "Test Task");
    assert_eq!(task.description, Some("Description".to_string()));
    assert_eq!(task.status, "pending");
}

#[test]
fn test_update_task() {
    let mut conn = establish_test_connection();
    let task = create_task(&mut conn, "Old Title", None, None);
    let updated_task = update_task(
        &mut conn,
        task.id,
        Some("New Title"),
        None,
        Some("completed"),
    );

    assert_eq!(updated_task.title, "New Title");
    assert_eq!(updated_task.status, "completed");
}

#[test]
fn test_delete_task() {
    let mut conn = establish_test_connection();
    let task = create_task(&mut conn, "Task to delete", None, None);
    let num_deleted = delete_task(&mut conn, task.id);

    assert_eq!(num_deleted, 1);
}
