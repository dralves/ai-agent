use anyhow::Result;
use clap::{ArgAction, CommandFactory, Parser, Subcommand};
use tracing::{debug, info};
use tracing_subscriber::EnvFilter;
mod config;
use config::AppConfig;
use std::path::Path;
mod db;
#[allow(dead_code)]
mod domain;
#[allow(dead_code)]
mod ports;

#[derive(Parser, Debug)]
#[command(name = "ai-trader", version, about = "AI Trading Agent CLI")] 
struct Cli {
    #[arg(short, long, action = ArgAction::Count, help = "Increase verbosity (-v, -vv)")]
    verbose: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Run the trading agent
    Run {
        /// Mode to run: paper or live
        #[arg(long, value_parser = ["paper", "live"], default_value = "paper")]
        mode: String,
    },

    /// Print the version and exit
    Version,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let cli = Cli::parse();

    let default_level = match cli.verbose {
        0 => "info",
        1 => "debug",
        _ => "trace",
    };

    let filter = std::env::var("RUST_LOG").unwrap_or_else(|_| default_level.to_string());
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new(filter))
        .with_target(false)
        .init();

    match cli.command {
        Some(Commands::Run { mode }) => {
            let mut app_config = AppConfig::load()?;
            // CLI overrides config
            app_config.mode = mode;
            info!(mode = %app_config.mode, "starting ai-trader");

            // Initialize database (SQLx SQLite)
            let db_url = app_config.database_url.clone();
            if let Some(path_str) = db_url
                .strip_prefix("sqlite://")
                .or_else(|| db_url.strip_prefix("sqlite:"))
            {
                if !path_str.contains(":memory:") {
                    if let Some(parent) = Path::new(path_str).parent() {
                        if !parent.as_os_str().is_empty() {
                            std::fs::create_dir_all(parent)?;
                        }
                    }
                }
            }
            let pool = db::connect(&db_url).await?;
            match &pool {
                db::DatabasePool::Sqlite(_) => debug!("database connected (sqlite)"),
                db::DatabasePool::Postgres(_) => debug!("database connected (postgres)"),
            }

            // Placeholder: M0 only ensures the binary runs
            debug!("runtime initialized");
        }
        Some(Commands::Version) => {
            println!("{}", env!("CARGO_PKG_VERSION"));
        }
        None => {
            // Print help and exit successfully
            let mut cmd = Cli::command();
            cmd.print_help().ok();
            println!();
        }
    }

    Ok(())
}


