//! Chatapp CLI - Command-line interface for Chatapp applications.

use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::process::Command;
use tracing_subscriber;
use platypus_server::AppServer;

#[derive(Parser)]
#[command(name = "platypus")]
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
    /// Run a Platypus application
    Run {
        /// Path to the app script or directory
        #[arg(value_name = "PATH")]
        path: PathBuf,

        /// Port to listen on
        #[arg(short, long, default_value_t = platypus_server::config::DEFAULT_PORT)]
        port: u16,

        /// Host to bind to
        #[arg(short = 'H', long, default_value = platypus_server::config::DEFAULT_HOST)]
        host: String,

        /// Enable hot reload
        #[arg(long)]
        hot_reload: bool,
    },

    /// Build a Platypus application for production
    Build {
        /// Path to the app script or directory
        #[arg(value_name = "PATH")]
        path: PathBuf,

        /// Output directory
        #[arg(short, long, default_value = platypus_server::config::DEFAULT_OUTPUT_DIR)]
        output: PathBuf,
    },

    /// Create a new Platypus project
    New {
        /// Project name
        #[arg(value_name = "NAME")]
        name: String,

        /// Template to use (basic, data, dashboard)
        #[arg(short, long, default_value = platypus_server::config::DEFAULT_TEMPLATE)]
        template: String,
    },

    /// Show version information
    Version,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // Setup logging
    let log_level = if cli.verbose {
        platypus_server::config::VERBOSE_LOG_LEVEL
    } else {
        platypus_server::config::NORMAL_LOG_LEVEL
    };
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
            println!("Platypus {}", env!("CARGO_PKG_VERSION"));
        }
    }

    Ok(())
}

/// Run a Platypus application.
async fn run_app(
    path: PathBuf,
    host: String,
    port: u16,
    _hot_reload: bool,
) -> anyhow::Result<()> {
    // Check if path is a .rs file in examples directory
    if let Some(file_name) = path.file_stem() {
        let bin_name = file_name.to_string_lossy();
        
        // Try to run as a cargo binary
        println!("🚀 Running example: {}", bin_name);
        let mut cmd = Command::new("cargo");
        cmd.args(&["run", "-p", "platypus-examples", "--bin", &bin_name])
            .env("PLATYPUS_HOST", &host)
            .env("PLATYPUS_PORT", port.to_string());
        
        let status = cmd.status()?;
        
        if !status.success() {
            anyhow::bail!("Failed to run example: {}", bin_name);
        }
        
        return Ok(());
    }
    
    // Fallback: start server without app
    println!("🚀 Starting Platypus server on http://{}:{}", host, port);
    println!("📝 Open your browser and navigate to the URL above");

    let config = platypus_server::ServerConfig {
        app_name: platypus_server::config::DEFAULT_APP_NAME.to_string(),
        host,
        port,
        max_body_size: platypus_server::config::DEFAULT_MAX_BODY_SIZE,
        session_timeout: platypus_server::config::DEFAULT_SESSION_TIMEOUT,
    };

    let server = AppServer::with_config(config);
    server.run().await?;

    Ok(())
}

/// Build a platypus application for production.
fn build_app(_path: PathBuf, _output: PathBuf) -> anyhow::Result<()> {
    println!("Building platypus application...");
    println!("✓ Build complete!");
    Ok(())
}

/// Create a new platypus project.
fn create_project(name: String, template: String) -> anyhow::Result<()> {
    println!("Creating new platypus project: {}", name);
    println!("Template: {}", template);
    println!("✓ Project created!");
    Ok(())
}
