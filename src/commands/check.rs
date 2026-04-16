use owo_colors::OwoColorize;
use std::fs;
use std::path::Path;

use crate::error::Result;
use crate::shared::configuration::{AnzarConfiguration, DatabaseDriver};
use crate::shared::support;

pub fn run(verbose: bool) -> Result<()> {
    println!("{}", "Running checks...".dimmed());

    check_git();
    check_yaml(verbose);
    check_compose(verbose)?;

    Ok(())
}

fn check_git() {
    let exists = Path::new(".git").exists();
    println!();
    println!("  Git");
    support::print_result("Repository", exists, None);
}

fn check_yaml(verbose: bool) {
    let candidates = ["anzar.yaml", "anzar.yml"];
    let path = candidates.iter().find(|p| Path::new(p).exists());

    println!();
    println!("  Config (anzar.yml)");
    let path = match path {
        Some(p) => p,
        None => {
            support::print_result("File", false, Some("no anzar.yml found"));
            return;
        }
    };

    let raw = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            support::print_result("Syntax", false, Some(&e.to_string()));
            return;
        }
    };

    match serde_yaml::from_str::<AnzarConfiguration>(&raw) {
        Ok(config) => {
            support::print_result("Syntax", true, None);
            // if verbose {
            //     println!("    {} {}", "url:".dimmed(), config.app.url.cyan());
            //     println!("    {} {}", "env:".dimmed(), config.app.environment.cyan());
            // }
            check_config_values(verbose, &config);
        }
        Err(e) => {
            support::print_result("Syntax", false, Some(&e.to_string()));
        }
    }
}

fn check_config_values(verbose: bool, config: &AnzarConfiguration) {
    // app.url should be a valid URL
    let url_ok = config.app.url.starts_with("http://") || config.app.url.starts_with("https://");
    support::print_result(
        "App URL",
        url_ok,
        Some("must start with http:// or https://"),
    );
    if verbose {
        println!("    {} {}", "url:".dimmed(), config.app.url.cyan());
    }

    // app.environment should be one of the known values
    let valid_envs = ["development", "staging", "production"];
    let env_ok = valid_envs.contains(&config.app.environment.as_str());
    support::print_result(
        "Environment",
        env_ok,
        Some("must be development, staging, or production"),
    );
    if verbose {
        println!("    {} {}", "url:".dimmed(), config.app.environment.cyan());
    }

    // security.secret_key should not be empty or too short
    let key_ok = config.security.secret_key.len() >= 32;
    support::print_result(
        "Secret key length",
        key_ok,
        Some("must be at least 32 characters"),
    );

    // if https is enabled, cert and key paths must be set
    if config.server.https.enabled {
        let certs_ok =
            config.server.https.cert_path.is_some() && config.server.https.key_path.is_some();
        support::print_result(
            "HTTPS certs",
            certs_ok,
            Some("cert_path and key_path required when https is enabled"),
        );
    }

    // password min < max
    let pwd = &config.auth.password.requirements;
    let pwd_ok = pwd.min_length < pwd.max_length;
    support::print_result(
        "Password length range",
        pwd_ok,
        Some("min_length must be less than max_length"),
    );
}

fn check_compose(verbose: bool) -> Result<()> {
    let path = match [
        "compose.yaml",
        "compose.yml",
        "docker-compose.yaml",
        "docker-compose.yml",
    ]
    .iter()
    .find(|p| Path::new(p).exists())
    {
        Some(p) => p,
        None => {
            support::print_result("Compose file", false, Some("no compose.yaml found"));
            return Ok(());
        }
    };

    println!();
    println!("  Compose ({path})");
    let raw = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            support::print_result("Syntax", false, Some(&e.to_string()));
            return Ok(());
        }
    };

    // Level 1 — valid YAML
    let doc: serde_yaml::Value = match serde_yaml::from_str(&raw) {
        Ok(v) => {
            support::print_result("Syntax", true, None);
            v
        }
        Err(e) => {
            support::print_result("Syntax", false, Some(&e.to_string()));
            return Ok(()); // no point checking further
        }
    };

    let services = &doc["services"];

    let config = support::load_config()?;
    let active_services: Vec<&str> = ["db", "cache"]
        .iter()
        .copied()
        .filter(|&s| s != "db" || config.database.driver != DatabaseDriver::SQLite)
        .collect();

    // Level 2 — required services exist
    for &service in &active_services {
        let ok = !services[service].is_null();
        support::print_result(
            &format!("Service: {}", service),
            ok,
            (!ok).then_some(&format!("missing '{}' service", service)),
        );
    }

    // Level 3 — services have an image or build defined
    for service in &active_services {
        let has_image = !services[service]["image"].is_null();
        let has_build = !services[service]["build"].is_null();
        let ok = has_image || has_build;
        support::print_result(
            &format!("{} has image or build", service),
            ok,
            Some("service must define either 'image' or 'build'"),
        );
    }

    if verbose {
        // print the image names if present
        for service in &active_services {
            if let Some(image) = services[service]["image"].as_str() {
                println!("    {} {}", format!("{}:", service).dimmed(), image.cyan());
            }
        }
    }

    Ok(())
}
