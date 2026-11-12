mod halo_core;
mod halo_protocol;

use halo_core::keypair::Keypair;
use std::error::Error;
use anyhow::{Result};
use tracing::{event, Level};
use tracing_subscriber::EnvFilter;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name="halod")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Publish a video from a file, using MoQ
    Publish {video_path: String},

    /// Subscribe to a MoQ stream using a ticket
    Subscribe {ticket: String},
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    // Enable tracing display on terminal. The EnvFilter enables us to add DEBUG output
    // by setting the env variable RUST_LOG=debug. By default we'll display from severity INFO only.
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));
    tracing_subscriber::fmt().with_writer(std::io::stderr).with_env_filter(filter).init();

    event!(Level::INFO, "starting halod 0.0.1");

    // Generate/use an ed25519 keypair. The public key will be used as NodeID.
    let k = Keypair::new()?;
    event!(Level::INFO, "pubkey for this node is {k}");

    match cli.command {
        Commands::Publish { video_path } => {
            eprintln!("Publishing {video_path}");
        },
        Commands::Subscribe { ticket } => {
            // Printing anything to stdout can corrupt the pipe to ffplay
            eprintln!("Subscribing to {ticket}");

            // Yes we're not using a ticket but it's easier and we'll implement it later
            // I just want to pub/sub right now
            halo_core::subscriber::subscribe(halo_protocol::node::parse_ip_port(&ticket)?).await?;
        }
    }

    Ok(())
}