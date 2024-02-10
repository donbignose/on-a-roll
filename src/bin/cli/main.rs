mod projects;
mod tasks;
use clap::{Parser, Subcommand};
use on_a_roll::db::connection::establish_connection;

#[derive(Debug, Parser)]
#[command(name = "roll")]
#[command(author, version, about, long_about = None)]
struct Cli {
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

fn main() {
    let cli = Cli::parse();
    let mut conn = establish_connection();

    match cli.command {
        Commands::Task(task_args) => {
            tasks::handle_task_args(task_args, &mut conn);
        }
        Commands::Project(project_args) => {
            projects::handle_project_args(project_args, &mut conn);
        }
    }
}
