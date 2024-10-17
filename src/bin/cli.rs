use clap::Parser;
use on_a_roll::{
    cli::{run_cli, Cli},
    db::connection::establish_connection,
};

fn main() {
    let cli = Cli::parse();
    let mut conn = establish_connection();

    run_cli(cli, &mut conn);
}
