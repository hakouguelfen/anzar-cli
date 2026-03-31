use std::fs;

use crate::dialoger::{cache::select_cache, database::select_database, strategy::select_strategy};

const CONFIG_TEMPLATE: &str = include_str!("../templates/configuration.yml");
const COMPOSE_TEMPLATE: &str = include_str!("../templates/compose.yml");

pub fn run(name: Option<String>) {
    if !std::path::Path::new(".git").exists() {
        eprintln!("Error: not a git repository. Run `git init` first.");
        std::process::exit(1);
    }

    let project_name = name.unwrap_or_else(|| "my-project".to_string());
    println!("Initializing project: {}", project_name);
    //

    let name: String = dialoguer::Input::new()
        .with_prompt("Your app name?")
        .interact_text()
        .unwrap();

    let (db_driver, db_uri) = select_database();
    let (cache_driver, cache_uri) = select_cache();
    let strategy = select_strategy();

    let compose_content = COMPOSE_TEMPLATE.replace("{{NAME}}", &name);
    let config_content = CONFIG_TEMPLATE
        .replace("{{DATABASE_DRIVER}}", &db_driver.to_string())
        .replace("{{DATABASE_URI}}", &db_uri)
        .replace("{{CACHE_DRIVER}}", &cache_driver.to_string())
        .replace("{{CACHE_URI}}", &cache_uri)
        .replace("{{STRATEGY}}", &strategy.to_string());

    match fs::write("anzar.yml", config_content) {
        Ok(_) => println!("Created anzar.yml"),
        Err(e) => eprintln!("Failed to create file: {}", e),
    }

    match fs::write("docker-compose.yml", compose_content) {
        Ok(_) => println!("Created docker-compose.yml"),
        Err(e) => eprintln!("Failed to create file: {}", e),
    }
}
