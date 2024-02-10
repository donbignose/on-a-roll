use clap::{Args, Subcommand};
use diesel::prelude::*;
use on_a_roll::models::Project;

#[derive(Debug, Args)]
pub struct ProjectArgs {
    #[command(subcommand)]
    command: ProjectCommands,
}
#[derive(Debug, Subcommand)]
enum ProjectCommands {
    /// Add a new project
    Add {
        /// Project title
        title: Option<String>,
        /// Optional project description
        description: Option<String>,
        /// Optional project status, defaults to 'Planning'
        status: Option<String>,
    },
    /// Update an existing project
    Update {
        /// Project id of project to update
        #[arg(required = true)]
        project_id: i32,
        /// New project title
        #[arg(short, long)]
        title: Option<String>,
        /// New project description
        #[arg(short, long)]
        description: Option<String>,
        /// New project status
        #[arg(short, long)]
        status: Option<String>,
    },
    /// Delete an existing project
    Delete {
        /// project id of project to delete
        #[arg(required = true)]
        project_id: i32,
    },
    /// Read an existing project
    Read {
        /// project id of project to view
        #[arg(required = true)]
        project_id: i32,
    },
    /// List all projects
    List,
}
fn handle_project_add(
    conn: &mut SqliteConnection,
    title: Option<String>,
    description: Option<String>,
    status: Option<String>,
) {
    println!(
        "Adding project: {:?} with description: {:?} and status: {:?}",
        title, description, status
    );
    match Project::create(
        conn,
        title.as_deref(),
        description.as_deref(),
        status.as_deref(),
    ) {
        Ok(project) => println!("Project created with id: {}", project.id),
        Err(e) => eprintln!("Error creating project: {}", e),
    }
}

fn handle_project_update(
    conn: &mut SqliteConnection,
    project_id: i32,
    title: Option<String>,
    description: Option<String>,
    status: Option<String>,
) {
    println!(
        "Updating project: {} with title: {:?}, description: {:?} and status: {:?}",
        project_id, title, description, status
    );
    match Project::update(
        conn,
        project_id,
        title.as_deref(),
        description.as_deref(),
        status.as_deref(),
    ) {
        Ok(project) => println!("Project updated: {:?}", project),
        Err(e) => eprintln!("Error updating project: {}", e),
    }
}
fn handle_project_delete(conn: &mut SqliteConnection, task_id: i32) {
    println!("Deleting project: {}", task_id);
    match Project::delete(conn, task_id) {
        Ok(amount) => println!("Deleted {} project(s)", amount),
        Err(e) => eprintln!("Error deleting project: {}", e),
    }
}

fn handle_project_read(conn: &mut SqliteConnection, task_id: i32) {
    match Project::find(conn, task_id) {
        Ok(project) => println!("Task found: {:?}", project),
        Err(diesel::result::Error::NotFound) => eprintln!("project not found"),
        Err(e) => eprintln!("Error finding project: {}", e),
    }
}

fn handle_project_list(conn: &mut SqliteConnection) {
    println!("Listing projects");
    match Project::list(conn) {
        Ok(projects) => {
            if projects.is_empty() {
                println!("No projects found");
            } else {
                for project in projects {
                    println!("{:?}", project);
                }
            }
        }
        Err(e) => eprintln!("Error listing projects: {}", e),
    }
}
pub fn handle_project_args(args: ProjectArgs, connection: &mut SqliteConnection) {
    match args.command {
        ProjectCommands::Add {
            title,
            description,
            status,
        } => handle_project_add(connection, title, description, status),
        ProjectCommands::Update {
            project_id,
            title,
            description,
            status,
        } => handle_project_update(connection, project_id, title, description, status),
        ProjectCommands::Delete { project_id } => handle_project_delete(connection, project_id),
        ProjectCommands::Read { project_id } => handle_project_read(connection, project_id),
        ProjectCommands::List => handle_project_list(connection),
    }
}
