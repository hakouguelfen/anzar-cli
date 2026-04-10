use crate::{
    error::Result,
    shared::{
        configuration::{AuthStrategy, DatabaseDriver},
        constants, support,
    },
};
use std::{fs, path::Path};

use owo_colors::OwoColorize;

pub fn run() -> Result<()> {
    let config = support::load_config()?;

    if config.database.driver == DatabaseDriver::MongoDB {
        support::print_result(
            "MongoDB is not supported",
            false,
            Some("switch to SQLite or PostgreSQL in anzar.yml"),
        );
        return Ok(());
    }

    let migrations_dir = Path::new("migrations");
    fs::create_dir_all(migrations_dir)?;

    // Check for existing anzar migrations
    let already_exists = fs::read_dir(migrations_dir)
        .map(|entries| {
            entries
                .filter_map(|e| e.ok())
                .any(|e| e.file_name().to_string_lossy().contains("anzar_create_"))
        })
        .unwrap_or(false);

    if already_exists {
        support::print_result(
            "Migrations already exist",
            false,
            Some("delete existing anzar_create_* files to regenerate"),
        );
        return Ok(());
    }

    let timestamp = chrono::Utc::now().format("%Y%m%d%H%M%S");

    let files = match config.auth.strategy {
        AuthStrategy::Session => constants::SESSION_TABLES,
        AuthStrategy::Jwt => constants::JWT_TABLES,
    };

    println!();
    for (content, name) in files {
        let filename = format!("{}/{}_{}.sql", migrations_dir.display(), timestamp, name);
        match fs::write(&filename, content) {
            Ok(_) => println!("{} {}", "✓ Created".green().bold(), filename.cyan()),
            Err(e) => eprintln!("{} {}", "✗ Failed to create file:".red().bold(), e),
        }
    }

    println!();
    println!(
        "  {} run {} to apply",
        "→".cyan().bold(),
        "anzar migrate".cyan()
    );

    Ok(())
}
