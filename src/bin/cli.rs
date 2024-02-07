use clap::{Parser, Subcommand};
use on_a_roll::establish_connection;
use on_a_roll::models::Task;

#[derive(Debug, Parser)]
#[command(name = "roll")]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Add a new task
    #[command(arg_required_else_help = true)]
    Add {
        /// Task title
        title: String,
        /// Optional task description
        description: Option<String>,
        /// Optional task status, defaults to 'pending'
        status: Option<String>,
    },
    /// Update an existing task
    Update {
        /// Task id of task to update
        task_id: i32,
        /// New task title
        title: Option<String>,
        /// New task description
        description: Option<String>,
        /// New task status
        status: Option<String>,
    },
    /// Delete an existing task
    Delete {
        /// Task id of task to delete
        task_id: i32,
    },
    /// Read an existing task
    Read {
        /// Task id of task to view
        task_id: i32,
    },
    /// List all tasks
    List,
}

fn main() {
    let cli = Cli::parse();
    let mut conn = establish_connection();

    match cli.command {
        Commands::Add {
            title,
            description,
            status,
        } => {
            println!(
                "Adding task: {} with description: {:?} and status: {:?}",
                title, description, status
            );
            match Task::create(&mut conn, &title, description.as_deref(), status.as_deref()) {
                Ok(task) => println!("Task created with id: {}", task.id),
                Err(e) => eprintln!("Error creating task: {}", e),
            }
        }
        Commands::Update {
            task_id,
            title,
            description,
            status,
        } => {
            println!(
                "Updating task: {} with title: {:?}, description: {:?} and status: {:?}",
                task_id, title, description, status
            );
            match Task::update(
                &mut conn,
                task_id,
                title.as_deref(),
                description.as_deref(),
                status.as_deref(),
            ) {
                Ok(task) => println!("Task updated: {:?}", task),
                Err(e) => eprintln!("Error updating task: {}", e),
            }
        }
        Commands::Delete { task_id } => {
            println!("Deleting task: {}", task_id);
            match Task::delete(&mut conn, task_id) {
                Ok(amount) => println!("Deleted {} task(s)", amount),
                Err(e) => eprintln!("Error deleting task: {}", e),
            }
        }
        Commands::Read { task_id } => match Task::find(&mut conn, task_id) {
            Ok(task) => println!("Task found: {:?}", task),
            Err(diesel::result::Error::NotFound) => eprintln!("Task not found"),
            Err(e) => eprintln!("Error finding task: {}", e),
        },
        Commands::List => {
            println!("Listing tasks");
            match Task::list(&mut conn) {
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
    }
}
