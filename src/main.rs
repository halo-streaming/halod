mod halo_core;

use std::error::Error;
use halo_core::keypair::Keypair;

fn main() -> Result<(), Box<dyn Error>> {
    println!("halod daemon version 0.0.1");

    // Generate/use an ed25519 keypair. The public key will be used as EndpointID.    
    let k = Keypair::new()?;
    println!("{k}");
    Ok(())
}
