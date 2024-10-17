use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub fn run_migrations(connection: &mut SqliteConnection) {
    connection.run_pending_migrations(MIGRATIONS).unwrap();
}
pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut connection = SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
    run_migrations(&mut connection);
    connection
}
