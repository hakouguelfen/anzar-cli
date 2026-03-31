use dialoguer::Select;

use crate::{models::database::DatabaseDriver, theme::theme};

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

    let db = databases[choice].clone();
    let uri = match db {
        DatabaseDriver::MongoDB => "mongodb://db:27017/default",
        DatabaseDriver::PostgreSQL => todo!(),
        DatabaseDriver::SQLite => "sqlite::memory",
    };

    (db, uri.to_string())
}
