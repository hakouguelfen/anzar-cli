use clap::{Parser, Subcommand};

mod commands;
mod dialoger;
mod models;
mod theme;

#[derive(Parser)]
#[command(name = "anzar")]
#[command(about = "Anzar is a lightweight authentication and authorization framework that runs as a separate microservice", long_about = None)]
#[command(author, version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init {
        #[arg(short, long)]
        name: Option<String>,
    },
    Check {
        #[arg(short, long)]
        verbose: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { name } => commands::init::run(name),
        Commands::Check { verbose } => commands::check::run(verbose),
    }
}
