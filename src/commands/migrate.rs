use crate::{
    error::{Error, Result},
    shared::{
        configuration::{AnzarConfiguration, DatabaseDriver},
        support,
    },
};
use owo_colors::OwoColorize;
use sqlx::{Executor, Pool, SqlitePool, migrate::Migrator, postgres::PgPoolOptions};

enum DatabasePool {
    Sqlite(Pool<sqlx::Sqlite>),
    Postgres(Pool<sqlx::Postgres>),
}

pub async fn run() -> Result<()> {
    let config = support::load_config()?;

    let database_pool: DatabasePool = match connect(config).await {
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

    let path = std::path::Path::new("migrations");
    if !path.exists() {
        support::print_result(
            "Migrations already exist",
            false,
            Some("delete existing anzar_create_* files to regenerate"),
        );
        return Ok(());
    }

    let migrator = Migrator::new(path).await?;
    let response = match database_pool {
        DatabasePool::Sqlite(pool) => migrator.run(&pool).await,
        DatabasePool::Postgres(pool) => migrator.run(&pool).await,
    };

    match response {
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

async fn connect(config: AnzarConfiguration) -> Result<DatabasePool, Error> {
    match config.database.driver {
        DatabaseDriver::MongoDB => {
            support::print_result(
                "MongoDB is not supported",
                false,
                Some("switch to SQLite or PostgreSQL in anzar.yml"),
            );
            Err(Error::InvalidConfig {
                key: "database.driver".to_string(),
                reason: "MongoDB is not supported".to_string(),
            })
        }
        DatabaseDriver::SQLite => {
            println!();
            support::print_result("Connecting to database", true, None);

            let conn = config.database.connection_string;

            let pool = SqlitePool::connect(&conn)
                .await
                .map_err(|e| Error::Other(e.to_string()))?;

            pool.execute("PRAGMA foreign_keys = ON;")
                .await
                .map_err(|e| Error::Other(e.to_string()))?;

            Ok(DatabasePool::Sqlite(pool))
        }
        DatabaseDriver::PostgreSQL => {
            println!();
            support::print_result("Connecting to database", true, None);

            let conn = config
                .database
                .connection_string
                .replace("@db", "@localhost");

            let pool = PgPoolOptions::new()
                .max_connections(5)
                .connect(&conn)
                .await
                .map_err(|e| Error::Other(e.to_string()))?;

            Ok(DatabasePool::Postgres(pool))
        }
    }
}
