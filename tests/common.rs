use diesel::prelude::*;
use on_a_roll::db::connection::run_migrations;

pub fn establish_test_connection() -> SqliteConnection {
    let mut connection =
        SqliteConnection::establish(":memory:").expect("Error creating in-memory database");
    run_migrations(&mut connection);
    connection
}
