use owo_colors::OwoColorize;
use std::fs;
use std::path::Path;

use crate::models::configuration::AnzarConfiguration;

pub fn run(verbose: bool) {
    println!("{}", "Running checks...".dimmed());
    println!();

    check_git(verbose);
    check_yaml(verbose);
}

fn check_git(verbose: bool) {
    let exists = Path::new(".git").exists();
    print_result("Git repository", exists, None);
    if verbose && exists {
        println!("    {}", "found .git directory".dimmed());
    }
}

fn check_yaml(verbose: bool) {
    let candidates = ["anzar.yaml", "anzar.yml"];
    let path = candidates.iter().find(|p| Path::new(p).exists());

    let path = match path {
        Some(p) => p,
        None => {
            print_result("Config file", false, Some("no anzar.yml found"));
            return;
        }
    };

    let raw = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            print_result("Config file", false, Some(&e.to_string()));
            return;
        }
    };

    if verbose {
        println!("    {}", format!("reading {}", path).dimmed());
    }

    match serde_yaml::from_str::<AnzarConfiguration>(&raw) {
        Ok(config) => {
            print_result("Config file", true, None);
            if verbose {
                println!("    {} {}", "url:".dimmed(), config.app.url.cyan());
                println!("    {} {}", "env:".dimmed(), config.app.environment.cyan());
            }
            check_config_values(&config);
        }
        Err(e) => {
            print_result("Config file", false, Some(&e.to_string()));
        }
    }
}

fn check_config_values(config: &AnzarConfiguration) {
    // app.url should be a valid URL
    let url_ok = config.app.url.starts_with("http://") || config.app.url.starts_with("https://");
    print_result(
        "App URL",
        url_ok,
        Some("must start with http:// or https://"),
    );

    // app.environment should be one of the known values
    let valid_envs = ["development", "staging", "production"];
    let env_ok = valid_envs.contains(&config.app.environment.as_str());
    print_result(
        "Environment",
        env_ok,
        Some("must be development, staging, or production"),
    );

    // security.secret_key should not be empty or too short
    let key_ok = config.security.secret_key.len() >= 32;
    print_result(
        "Secret key length",
        key_ok,
        Some("must be at least 32 characters"),
    );

    // if https is enabled, cert and key paths must be set
    if config.server.https.enabled {
        let certs_ok =
            config.server.https.cert_path.is_some() && config.server.https.key_path.is_some();
        print_result(
            "HTTPS certs",
            certs_ok,
            Some("cert_path and key_path required when https is enabled"),
        );
    }

    // password min < max
    let pwd = &config.auth.password.requirements;
    let pwd_ok = pwd.min_length < pwd.max_length;
    print_result(
        "Password length range",
        pwd_ok,
        Some("min_length must be less than max_length"),
    );
}

fn print_result(label: &str, passed: bool, hint: Option<&str>) {
    if passed {
        println!("  {} {}", "✓".green().bold(), label);
    } else {
        match hint {
            Some(h) => println!(
                "  {} {} {}",
                "✗".red().bold(),
                label.red(),
                format!("({})", h).dimmed()
            ),
            None => println!("  {} {}", "✗".red().bold(), label.red()),
        }
    }
}
