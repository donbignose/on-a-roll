use crate::models::Task;
use clap::{Args, Subcommand};
use diesel::prelude::*;

#[derive(Debug, Args)]
pub struct TaskArgs {
    #[command(subcommand)]
    command: TaskCommands,
}

#[derive(Debug, Subcommand)]
enum TaskCommands {
    /// Add a new task
    Add {
        /// Task title
        title: Option<String>,
        /// Optional task description
        description: Option<String>,
        /// Optional task status, defaults to 'pending'
        status: Option<String>,
        /// Optional project id
        project_id: Option<i32>,
    },
    /// Update an existing task
    Update {
        /// Task id of task to update
        #[arg(required = true)]
        task_id: i32,
        /// New task title
        #[arg(short, long)]
        title: Option<String>,
        /// New task description
        #[arg(short, long)]
        description: Option<String>,
        /// New task status
        #[arg(short, long)]
        status: Option<String>,
        /// New project id
        #[arg(short, long = "project")]
        project_id: Option<i32>,
    },
    /// Delete an existing task
    Delete {
        /// Task id of task to delete
        #[arg(required = true)]
        task_id: i32,
    },
    /// Read an existing task
    Read {
        /// Task id of task to view
        #[arg(required = true)]
        task_id: i32,
    },
    /// List all tasks
    List,
}

fn handle_task_add(
    conn: &mut SqliteConnection,
    title: Option<String>,
    description: Option<String>,
    status: Option<String>,
    project_id: Option<i32>,
) {
    println!(
        "Adding task: {:?} with description: {:?} and status: {:?}",
        title, description, status
    );
    match Task::create(
        conn,
        title.as_deref(),
        description.as_deref(),
        status.as_deref(),
        project_id,
    ) {
        Ok(task) => println!("Task created with id: {}", task.id),
        Err(e) => eprintln!("Error creating task: {}", e),
    }
}

fn handle_task_update(
    conn: &mut SqliteConnection,
    task_id: i32,
    title: Option<String>,
    description: Option<String>,
    status: Option<String>,
    project_id: Option<i32>,
) {
    println!(
        "Updating task: {} with title: {:?}, description: {:?} and status: {:?}",
        task_id, title, description, status
    );
    match Task::update(
        conn,
        task_id,
        title.as_deref(),
        description.as_deref(),
        status.as_deref(),
        project_id,
    ) {
        Ok(task) => println!("Task updated: {:?}", task),
        Err(e) => eprintln!("Error updating task: {}", e),
    }
}
fn handle_task_delete(conn: &mut SqliteConnection, task_id: i32) {
    println!("Deleting task: {}", task_id);
    match Task::delete(conn, task_id) {
        Ok(amount) => println!("Deleted {} task(s)", amount),
        Err(e) => eprintln!("Error deleting task: {}", e),
    }
}

fn handle_task_read(conn: &mut SqliteConnection, task_id: i32) {
    match Task::find(conn, task_id) {
        Ok(task) => println!("Task found: {:?}", task),
        Err(diesel::result::Error::NotFound) => eprintln!("Task not found"),
        Err(e) => eprintln!("Error finding task: {}", e),
    }
}

fn handle_task_list(conn: &mut SqliteConnection) {
    println!("Listing tasks");
    match Task::list(conn) {
        Ok(tasks) => {
            if tasks.is_empty() {
                println!("No tasks found");
            } else {
                for task in tasks {
                    println!("{:?}", task);
                }
            }
        }
        Err(e) => eprintln!("Error listing tasks: {}", e),
    }
}
pub fn handle_task_args(args: TaskArgs, connection: &mut SqliteConnection) {
    match args.command {
        TaskCommands::Add {
            title,
            description,
            status,
            project_id,
        } => handle_task_add(connection, title, description, status, project_id),
        TaskCommands::Update {
            task_id,
            title,
            description,
            status,
            project_id,
        } => handle_task_update(connection, task_id, title, description, status, project_id),
        TaskCommands::Delete { task_id } => handle_task_delete(connection, task_id),
        TaskCommands::Read { task_id } => handle_task_read(connection, task_id),
        TaskCommands::List => handle_task_list(connection),
    }
}
