use owo_colors::OwoColorize;
use std::fs;
use std::path::Path;

use crate::models::configuration::AnzarConfiguration;

pub fn run(verbose: bool) {
    println!("{}", "Running checks...".dimmed());

    check_git();
    check_yaml(verbose);
    check_compose(verbose);
}

fn check_git() {
    let exists = Path::new(".git").exists();
    println!();
    println!("  Git");
    print_result("Repository", exists, None);
}

fn check_yaml(verbose: bool) {
    let candidates = ["anzar.yaml", "anzar.yml"];
    let path = candidates.iter().find(|p| Path::new(p).exists());

    println!();
    println!("  Config (anzar.yml)");
    let path = match path {
        Some(p) => p,
        None => {
            print_result("File", false, Some("no anzar.yml found"));
            return;
        }
    };

    let raw = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            print_result("Syntax", false, Some(&e.to_string()));
            return;
        }
    };

    match serde_yaml::from_str::<AnzarConfiguration>(&raw) {
        Ok(config) => {
            print_result("Syntax", true, None);
            // if verbose {
            //     println!("    {} {}", "url:".dimmed(), config.app.url.cyan());
            //     println!("    {} {}", "env:".dimmed(), config.app.environment.cyan());
            // }
            check_config_values(verbose, &config);
        }
        Err(e) => {
            print_result("Syntax", false, Some(&e.to_string()));
        }
    }
}

fn check_config_values(verbose: bool, config: &AnzarConfiguration) {
    // app.url should be a valid URL
    let url_ok = config.app.url.starts_with("http://") || config.app.url.starts_with("https://");
    print_result(
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
    print_result(
        "Environment",
        env_ok,
        Some("must be development, staging, or production"),
    );
    if verbose {
        println!("    {} {}", "url:".dimmed(), config.app.environment.cyan());
    }

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

fn check_compose(verbose: bool) {
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
            print_result("Compose file", false, Some("no compose.yaml found"));
            return;
        }
    };

    println!();
    println!("  Compose (compose.yml)");
    let raw = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            print_result("Syntax", false, Some(&e.to_string()));
            return;
        }
    };

    // Level 1 — valid YAML
    let doc: serde_yaml::Value = match serde_yaml::from_str(&raw) {
        Ok(v) => {
            print_result("Syntax", true, None);
            v
        }
        Err(e) => {
            print_result("Syntax", false, Some(&e.to_string()));
            return; // no point checking further
        }
    };

    // Level 2 — required services exist
    let services = &doc["services"];

    let db_ok = !services["db"].is_null();
    print_result("Service: db", db_ok, Some("missing 'db' service"));

    let cache_ok = !services["cache"].is_null();
    print_result("Service: cache", cache_ok, Some("missing 'cache' service"));

    // Level 3 — services have an image or build defined
    for service in ["db", "cache"] {
        let has_image = !services[service]["image"].is_null();
        let has_build = !services[service]["build"].is_null();
        let ok = has_image || has_build;
        print_result(
            &format!("{} has image or build", service),
            ok,
            Some("service must define either 'image' or 'build'"),
        );
    }

    if verbose {
        // print the image names if present
        for service in ["db", "cache"] {
            if let Some(image) = services[service]["image"].as_str() {
                println!("    {} {}", format!("{}:", service).dimmed(), image.cyan());
            }
        }
    }
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
