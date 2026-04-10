use clap::{Parser, Subcommand};
use error::Result;

mod commands;
mod dialoger;
mod error;
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
    #[command(about = "Initialize Anzar for your project", long_about = None)]
    Init {
        #[arg(short, long)]
        name: Option<String>,
    },

    #[command(about = "Check current configuration and setup", long_about = None)]
    Check {
        #[arg(short, long)]
        verbose: bool,
    },

    #[command(about = "Show Anzar service status", long_about = None)]
    Status {},

    #[command(about = "Generate database schemas", long_about = None, visible_alias = "gen")]
    Generate {},

    #[command(about = "Apply database migrations", long_about = None)]
    Migrate {},
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { name } => commands::init::run(name),
        Commands::Check { verbose } => commands::check::run(verbose),
        Commands::Status {} => commands::status::run(),
        Commands::Generate {} => commands::generate::run(),
        Commands::Migrate {} => commands::migrate::run().await,
    }
}
