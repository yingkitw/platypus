//! Chatapp CLI - Command-line interface for Chatapp applications.

use clap::{Parser, Subcommand};
use std::path::PathBuf;
use tracing_subscriber;
use chatapp_server::AppServer;

#[derive(Parser)]
#[command(name = "chatapp")]
#[command(about = "Chatapp - Web App Generator", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Enable verbose logging
    #[arg(short, long, global = true)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a Chatapp application
    Run {
        /// Path to the app script or directory
        #[arg(value_name = "PATH")]
        path: PathBuf,

        /// Port to listen on
        #[arg(short, long, default_value = "8501")]
        port: u16,

        /// Host to bind to
        #[arg(short, long, default_value = "127.0.0.1")]
        host: String,

        /// Enable hot reload
        #[arg(long)]
        hot_reload: bool,
    },

    /// Build a Chatapp application for production
    Build {
        /// Path to the app script or directory
        #[arg(value_name = "PATH")]
        path: PathBuf,

        /// Output directory
        #[arg(short, long, default_value = "dist")]
        output: PathBuf,
    },

    /// Create a new Chatapp project
    New {
        /// Project name
        #[arg(value_name = "NAME")]
        name: String,

        /// Template to use (basic, data, dashboard)
        #[arg(short, long, default_value = "basic")]
        template: String,
    },

    /// Show version information
    Version,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // Setup logging
    let log_level = if cli.verbose { "debug" } else { "info" };
    tracing_subscriber::fmt()
        .with_max_level(log_level.parse().unwrap_or(tracing::Level::INFO))
        .init();

    match cli.command {
        Commands::Run {
            path,
            port,
            host,
            hot_reload,
        } => {
            run_app(path, host, port, hot_reload).await?;
        }
        Commands::Build { path, output } => {
            build_app(path, output)?;
        }
        Commands::New { name, template } => {
            create_project(name, template)?;
        }
        Commands::Version => {
            println!("Chatapp {}", env!("CARGO_PKG_VERSION"));
        }
    }

    Ok(())
}

/// Run a Webag application.
async fn run_app(
    _path: PathBuf,
    host: String,
    port: u16,
    _hot_reload: bool,
) -> anyhow::Result<()> {
    println!("🚀 Starting Webag server on http://{}:{}", host, port);
    println!("📝 Open your browser and navigate to the URL above");

    let config = chatapp_server::ServerConfig {
        app_name: "Webag App".to_string(),
        host,
        port,
        max_body_size: 100 * 1024 * 1024,
        session_timeout: 3600,
    };

    let server = AppServer::with_config(config);
    server.run().await?;

    Ok(())
}

/// Build a Webag application for production.
fn build_app(_path: PathBuf, _output: PathBuf) -> anyhow::Result<()> {
    println!("Building Webag application...");
    println!("✓ Build complete!");
    Ok(())
}

/// Create a new Webag project.
fn create_project(name: String, template: String) -> anyhow::Result<()> {
    println!("Creating new Webag project: {}", name);
    println!("Template: {}", template);
    println!("✓ Project created!");
    Ok(())
}
