use base32::Alphabet;
use ed25519_dalek::pkcs8::spki::der::pem::LineEnding;
use ed25519_dalek::pkcs8::{DecodePrivateKey, EncodePrivateKey};
use ed25519_dalek::{SigningKey, VerifyingKey};
use rand::rngs::OsRng;
use std::error::Error;
use std::fmt;
use std::fs;
use tracing::{event, Level};

/// An Ed25519 keypair. The public key is used as a NodeID.
pub struct Keypair {
    pubkey: VerifyingKey,
    privkey: SigningKey,
}

/// Saves a private key as PKCS#8 PEM format to specified path.
fn save_private_key(signing_key: &SigningKey, path: &str) -> Result<(), Box<dyn Error>> {
    let pem = signing_key.to_pkcs8_pem(LineEnding::LF)?;
    std::fs::write(path, pem.as_bytes())?;
    Ok(())
}

/// Loads a PKCS#8 PEM format private key from specified path.
fn load_private_key(path: &str) -> Result<SigningKey, Box<dyn Error>> {
    let pem = std::fs::read_to_string(path)?;
    let signing_key = SigningKey::from_pkcs8_pem(&pem)?;
    Ok(signing_key)
}

impl Keypair {
    /// Generates an Ed25519 keypair.
    /// If `node.key` already exists, we load it instead of generating a new one.
    /// Otherwise, generates a new keypair and stores it in `node.key`.
    pub fn new() -> Result<Self, Box<dyn Error>> {
        // Has a keypair already been generated at some point?
        match fs::metadata("node.key") {
            Ok(_) => {
                event!(Level::DEBUG, "A private key already exists!");
                let signing_key = load_private_key("node.key")?;
                let verifying_key = signing_key.verifying_key();
                Ok(Keypair {
                    pubkey: verifying_key,
                    privkey: signing_key,
                })
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                let signing_key = SigningKey::generate(&mut OsRng);
                let verifying_key = signing_key.verifying_key();

                // Save private key to disk as PEM format (like PGP keys)
                save_private_key(&signing_key, "node.key").unwrap();
                Ok(Keypair {
                    pubkey: verifying_key,
                    privkey: signing_key,
                })
            }
            Err(e) => {
                Err(e.into())
            }
        }
    }
}

impl fmt::Display for Keypair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // We could display private key.. if we wanted to.
        // We only display the public key we generated, to not expose any secrets.
        // The pubkey is displayed in a z-base32 encoding.
        write!(f, "{}", base32::encode(Alphabet::Z, &self.pubkey.to_bytes()))
    }
}
