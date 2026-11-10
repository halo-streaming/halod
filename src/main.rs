mod halo_core;

use halo_core::keypair::Keypair;
use std::error::Error;
use tracing::{event, Level};
use tracing_subscriber::EnvFilter;

fn main() -> Result<(), Box<dyn Error>> {
    // Enable tracing display on terminal. The EnvFilter enables us to add DEBUG output
    // by setting the env variable RUST_LOG=debug. By default we'll display from severity INFO only.
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));
    tracing_subscriber::fmt().with_env_filter(filter).init();

    event!(Level::INFO, "starting halod 0.0.1");

    // Generate/use an ed25519 keypair. The public key will be used as NodeID.
    let k = Keypair::new()?;
    event!(Level::INFO, "pubkey for this node is {k}");
    Ok(())
}
