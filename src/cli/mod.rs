mod projects;
mod tasks;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "roll")]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(arg_required_else_help = true)]
    Task(tasks::TaskArgs),
    #[command(arg_required_else_help = true)]
    Project(projects::ProjectArgs),
}

pub fn run_cli(cli: Cli, conn: &mut diesel::SqliteConnection) {
    match cli.command {
        Commands::Task(task_args) => {
            tasks::handle_task_args(task_args, conn);
        }
        Commands::Project(project_args) => {
            projects::handle_project_args(project_args, conn);
        }
    }
}
