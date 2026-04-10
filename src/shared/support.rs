use owo_colors::OwoColorize;
use std::{fs, path::Path};

use crate::{
    error::{Error, Result},
    shared::configuration::AnzarConfiguration,
};

pub fn load_config() -> Result<AnzarConfiguration> {
    let candidates = ["anzar.yaml", "anzar.yml"];
    let path = candidates.iter().find(|p| Path::new(p).exists());

    let path = match path {
        Some(p) => p,
        None => {
            print_result(
                "anzar.yml not found",
                false,
                Some("run `anzar init` to create one"),
            );
            return Err(Error::FileNotFound {
                path: std::path::PathBuf::from("anzar.yml"),
            });
        }
    };

    let raw = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            print_result("Failed to read anzar.yml", false, Some(&e.to_string()));
            return Err(Error::InvalidConfig {
                key: "key".to_string(),
                reason: e.to_string(),
            });
        }
    };

    serde_yaml::from_str::<AnzarConfiguration>(&raw).map_err(|e| {
        print_result(
            "Invalid anzar.yml",
            false,
            Some(&format!("parse error: {}", e)),
        );
        e.into()
    })
}

pub fn print_result(label: &str, passed: bool, hint: Option<&str>) {
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
