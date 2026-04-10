use crate::{
    commands::support,
    error::Result,
    models::{database::DatabaseDriver, strategy::AuthStrategy},
};
use std::{fs, path::Path};

use owo_colors::OwoColorize;

const CREATE_USERS: &str = include_str!("../templates/db/sqlite/create_users.sql");
const CREATE_ACCOUNTS: &str = include_str!("../templates/db/sqlite/create_accounts.sql");
const CREATE_SESSIONS: &str = include_str!("../templates/db/sqlite/create_sessions.sql");
const CREATE_REFRESH_TOKENS: &str =
    include_str!("../templates/db/sqlite/create_refresh_tokens.sql");
const CREATE_PASSWORD_RESET_TOKENS: &str =
    include_str!("../templates/db/sqlite/create_password_reset_tokens.sql");
const CREATE_EMAIL_VERIFICATION_TOKENS: &str =
    include_str!("../templates/db/sqlite/create_email_verification_tokens.sql");

pub fn run() -> Result<()> {
    let config = support::load_config()?;

    if config.database.driver == DatabaseDriver::MongoDB {
        support::print_result(
            "MongoDB is not supported",
            false,
            Some("switch to SQLite or Postgres in anzar.yml"),
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
        AuthStrategy::Session => vec![
            (CREATE_USERS, "anzar_create_users"),
            (CREATE_ACCOUNTS, "anzar_create_accounts"),
            (CREATE_SESSIONS, "anzar_create_sessions"),
            (CREATE_REFRESH_TOKENS, "anzar_create_refresh_tokens"),
            (
                CREATE_PASSWORD_RESET_TOKENS,
                "anzar_create_password_reset_tokens",
            ),
            (
                CREATE_EMAIL_VERIFICATION_TOKENS,
                "anzar_create_email_verification_tokens",
            ),
        ],
        AuthStrategy::Jwt => vec![(CREATE_USERS, "myauth_create_users")],
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
