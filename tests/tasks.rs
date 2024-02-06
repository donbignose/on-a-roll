mod common;
use common::establish_test_connection;
use on_a_roll::models::Task;

#[test]
fn test_create_task() {
    let mut conn = establish_test_connection();
    let task = Task::create(&mut conn, "Test Task", Some("Description"), None);
    match task {
        Ok(task) => {
            assert_eq!(task.title, "Test Task");
            assert_eq!(task.description, Some("Description".to_string()));
            assert_eq!(task.status, "pending");
        }
        Err(e) => panic!("Failed to create task: {}", e),
    }
}

#[test]
fn test_update_task() {
    let mut conn = establish_test_connection();
    let task = Task::create(&mut conn, "Old Title", None, None).unwrap();
    let updated_result = Task::update(
        &mut conn,
        task.id,
        Some("New Title"),
        None,
        Some("completed"),
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
    let task = Task::create(&mut conn, "Task to find", None, None).unwrap();
    let found_result = Task::find(&mut conn, &task.id);

    match found_result {
        Ok(found_task) => assert_eq!(found_task.title, "Task to find"),
        Err(e) => panic!("Failed to find task: {}", e),
    }
}

#[test]

fn test_delete_task() {
    let mut conn = establish_test_connection();
    let task = Task::create(&mut conn, "Task to delete", None, None).unwrap();
    let delete_result = Task::delete(&mut conn, task.id);

    match delete_result {
        Ok(num_deleted) => assert_eq!(num_deleted, 1),
        Err(e) => panic!("Failed to delete task: {}", e),
    }
}
