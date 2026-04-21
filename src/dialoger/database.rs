use crate::{shared::configuration::DatabaseDriver, theme::theme};
use dialoguer::Select;

pub fn select_database() -> (DatabaseDriver, String) {
    let databases: Vec<DatabaseDriver> = vec![
        DatabaseDriver::MongoDB,
        DatabaseDriver::PostgreSQL,
        DatabaseDriver::SQLite,
    ];
    let choice = Select::with_theme(&theme())
        .with_prompt("Select database")
        .items(&databases)
        .default(0)
        .interact()
        .unwrap();

    // TODO
    // Ask for DB_USER and DB_PASSWORD

    let db = databases[choice].clone();
    let uri = match db {
        DatabaseDriver::MongoDB => "mongodb://user:password@db:27017/mongodb?authSource=admin",
        DatabaseDriver::PostgreSQL => "postgres://postgres:password@db:5432/postgres",
        DatabaseDriver::SQLite => "file:default.db",
    };

    (db, uri.to_string())
}
