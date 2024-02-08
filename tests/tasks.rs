mod common;
use common::establish_test_connection;
use on_a_roll::models::Task;
use on_a_roll::models::{DEFAULT_TASK_STATUS, DEFAULT_TASK_TITLE};

#[test]
fn test_create_task() {
    let mut conn = establish_test_connection();
    let task = Task::create(
        &mut conn,
        Some("Test Task"),
        Some("Description"),
        None,
        None,
    );
    match task {
        Ok(task) => {
            assert_eq!(task.title, "Test Task");
            assert_eq!(task.description, Some("Description".to_string()));
            assert_eq!(task.status, DEFAULT_TASK_STATUS);
        }
        Err(e) => panic!("Failed to create task: {}", e),
    }
}

#[test]
fn test_update_task() {
    let mut conn = establish_test_connection();
    let task = Task::create(&mut conn, Some("Old Title"), None, None, None).unwrap();
    let updated_result = Task::update(
        &mut conn,
        task.id,
        Some("New Title"),
        None,
        Some("completed"),
        None,
    );

    match updated_result {
        Ok(updated_task) => {
            assert_eq!(updated_task.title, "New Title");
            assert_eq!(updated_task.status, "completed");
        }
        Err(e) => panic!("Failed to update task: {}", e),
    }
}

#[test]
fn test_find_task() {
    let mut conn = establish_test_connection();
    let task = Task::create(&mut conn, Some("Task to find"), None, None, None).unwrap();
    let found_result = Task::find(&mut conn, task.id);

    match found_result {
        Ok(found_task) => assert_eq!(found_task.title, "Task to find"),
        Err(e) => panic!("Failed to find task: {}", e),
    }
}

#[test]
fn test_list_tasks() {
    let mut conn = establish_test_connection();
    Task::create(&mut conn, Some("Task 1"), None, None, None).unwrap();
    Task::create(&mut conn, Some("Task 2"), None, None, None).unwrap();

    let tasks = Task::list(&mut conn);
    match tasks {
        Ok(tasks) => assert!(tasks.len() == 2),
        Err(e) => panic!("Failed to list tasks: {}", e),
    }
}

#[test]
fn test_list_tasks_no_tasks() {
    let mut conn = establish_test_connection();

    let tasks = Task::list(&mut conn);
    match tasks {
        Ok(tasks) => assert!(tasks.is_empty()),
        Err(e) => panic!("Failed to list tasks: {}", e),
    }
}
#[test]
fn test_delete_task() {
    let mut conn = establish_test_connection();
    let task = Task::create(&mut conn, Some("Task to delete"), None, None, None).unwrap();
    let delete_result = Task::delete(&mut conn, task.id);

    match delete_result {
        Ok(num_deleted) => assert_eq!(num_deleted, 1),
        Err(e) => panic!("Failed to delete task: {}", e),
    }
}

#[test]
fn test_create_default_task() {
    let mut conn = establish_test_connection();
    let task = Task::create(&mut conn, None, None, None, None).unwrap(); // Assuming title cannot be empty

    assert_eq!(task.title, DEFAULT_TASK_TITLE);
    assert_eq!(task.description, None);
    assert_eq!(task.status, DEFAULT_TASK_STATUS);
}

#[test]
fn test_update_task_error() {
    let mut conn = establish_test_connection();
    let result = Task::update(&mut conn, 9999, Some("Non-existent"), None, None, None); // Non-existent ID

    assert!(
        matches!(result, Err(diesel::result::Error::NotFound)),
        "Expected a NotFound error"
    );
}
#[test]
fn test_find_task_error() {
    let mut conn = establish_test_connection();
    let result = Task::find(&mut conn, 9999); // Non-existent ID

    assert!(
        matches!(result, Err(diesel::result::Error::NotFound)),
        "Expected a NotFound error"
    );
}

#[test]
fn test_delete_task_error() {
    let mut conn = establish_test_connection();
    let result = Task::delete(&mut conn, 9999); // Non-existent ID

    assert!(
        result.is_ok(),
        "Delete operation should not error even if no rows are affected"
    );
    assert_eq!(result.unwrap(), 0, "Expected 0 rows to be deleted");
}
