use std::fs;

use dialoguer::Confirm;
use owo_colors::OwoColorize;

use crate::{
    dialoger::{cache::select_cache, database::select_database, strategy::select_strategy},
    theme::theme,
};

const CONFIG_TEMPLATE: &str = include_str!("../templates/configuration.yml");
const COMPOSE_TEMPLATE: &str = include_str!("../templates/compose.yml");

pub fn run(app_name: Option<String>) {
    if !std::path::Path::new(".git").exists() {
        eprintln!(
            "{} not a git repository. Run {} first.",
            "Error:".red().bold(),
            "`git init`".yellow()
        );
        std::process::exit(1);
    }

    // Detect existing files
    if std::path::Path::new("compose.yml").exists() {
        let confirmation = Confirm::new()
            .with_prompt("compose.yml already exists. Overwrite?")
            .interact()
            .unwrap();

        if confirmation {
            let name: String = match app_name {
                Some(n) => n,
                None => dialoguer::Input::with_theme(&theme())
                    .with_prompt("Your app name?")
                    .interact_text()
                    .unwrap(),
            };
            println!("Initializing project: {}", name);
            let compose_content = COMPOSE_TEMPLATE.replace("{{NAME}}", &name);

            match fs::write("compose.yml", compose_content) {
                Ok(_) => println!("{} {}", "✓ Created".green().bold(), "compose.yml".cyan()),
                Err(e) => eprintln!("{} {}", "✗ Failed to create file:".red().bold(), e),
            }
        }
    }

    if std::path::Path::new("anzar.yml").exists() {
        let confirmation = Confirm::new()
            .with_prompt("anzar.yml already exists. Overwrite?")
            .interact()
            .unwrap();

        if confirmation {
            let (db_driver, db_uri) = select_database();
            let (cache_driver, cache_uri) = select_cache();
            let strategy = select_strategy();

            let config_content = CONFIG_TEMPLATE
                .replace("{{DATABASE_DRIVER}}", &db_driver.to_string())
                .replace("{{DATABASE_URI}}", &db_uri)
                .replace("{{CACHE_DRIVER}}", &cache_driver.to_string())
                .replace("{{CACHE_URI}}", &cache_uri)
                .replace("{{STRATEGY}}", &strategy.to_string());

            println!();
            match fs::write("anzar.yml", config_content) {
                Ok(_) => println!("{} {}", "✓ Created".green().bold(), "anzar.yml".cyan()),
                Err(e) => eprintln!("{} {}", "✗ Failed to create file:".red().bold(), e),
            }
        }
    }
}
