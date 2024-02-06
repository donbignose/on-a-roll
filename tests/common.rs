use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

fn run_migrations(connection: &mut SqliteConnection) {
    connection.run_pending_migrations(MIGRATIONS).unwrap();
}
pub fn establish_test_connection() -> SqliteConnection {
    let mut connection =
        SqliteConnection::establish(":memory:").expect("Error creating in-memory database");
    run_migrations(&mut connection);
    connection
}
