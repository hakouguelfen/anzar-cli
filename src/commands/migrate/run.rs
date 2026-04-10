use crate::{
    commands::support,
    error::{Error, Result},
    models::{configuration::AnzarConfiguration, database::DatabaseDriver},
};
use owo_colors::OwoColorize;
use sqlx::{Executor, Pool, Sqlite, SqlitePool};

pub async fn run() -> Result<()> {
    let config = support::load_config()?;

    println!();
    support::print_result("Connecting to database", true, None);

    let db: Pool<Sqlite> = match connect(config).await {
        Ok(pool) => pool,
        Err(e) => {
            support::print_result(
                "Failed to connect",
                false,
                Some(&format!("check connection_string in anzar.yml — {}", e)),
            );
            return Ok(());
        }
    };

    support::print_result("Running migrations", true, None);

    match sqlx::migrate!("./migrations").run(&db).await {
        Ok(_) => {
            support::print_result("Migrations applied", true, None);
            println!();
            println!("  {} your database is up to date", "→".cyan().bold());
        }
        Err(e) => {
            let hint = match &e {
                sqlx::migrate::MigrateError::VersionMissing(v) => {
                    format!("migration version {} is missing from ./migrations", v)
                }
                sqlx::migrate::MigrateError::Dirty(v) => {
                    format!(
                        "migration version {} failed previously — fix or revert it first",
                        v
                    )
                }
                _ => e.to_string(),
            };
            support::print_result("Migrations failed", false, Some(&hint));
            return Ok(());
        }
    }

    Ok(())
}

async fn connect(config: AnzarConfiguration) -> Result<Pool<Sqlite>, Error> {
    match config.database.driver {
        DatabaseDriver::MongoDB => {
            support::print_result(
                "MongoDB is not supported",
                false,
                Some("switch to SQLite or Postgres in anzar.yml"),
            );
            return Err(Error::InvalidConfig {
                key: "database.driver".to_string(),
                reason: "MongoDB is not supported".to_string(),
            });
        }
        DatabaseDriver::PostgreSQL => {
            let cnx_string = config.database.connection_string;

            let pool = SqlitePool::connect(&cnx_string)
                .await
                .map_err(|e| Error::Other(e.to_string()))?;

            pool.execute("PRAGMA foreign_keys = ON;")
                .await
                .map_err(|e| Error::Other(e.to_string()))?;

            Ok(pool)
        }
        DatabaseDriver::SQLite => todo!(),
    }
}
