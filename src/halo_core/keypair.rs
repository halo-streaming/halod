use ed25519_dalek::{SigningKey, VerifyingKey};
use ed25519_dalek::pkcs8::{DecodePrivateKey, EncodePrivateKey};
use ed25519_dalek::pkcs8::spki::der::pem::LineEnding;
use rand::rngs::OsRng;
use std::fmt;
use base32::Alphabet;
use std::error::Error;
use std::fs;

pub struct Keypair {
    public: VerifyingKey, 
    private: SigningKey,
}

// Save a private key as PKCS#8 PEM format to specified path
fn save_private_key(signing_key: &SigningKey, path: &str) -> Result<(), Box<dyn Error>> {
    let pem = signing_key.to_pkcs8_pem(LineEnding::LF)?;
    std::fs::write(path, pem.as_bytes())?;
    Ok(())
}

// Load a PKCS#8 PEM format private key from specified path
fn load_private_key(path: &str) -> Result<SigningKey, Box<dyn Error>> {
    let pem = std::fs::read_to_string(path)?;
    let signing_key = SigningKey::from_pkcs8_pem(&pem)?;
    Ok(signing_key)
}

impl Keypair {
    // Generate an ed25519 keypair.
    // If a private key already exists on the system, we use it instead. 
    pub fn new() -> Self {
        // Has a keypair already been generated at some point?
        if fs::metadata("node.key").is_ok() {
            println!("A private key already exists!");
            let signing_key = load_private_key("node.key").unwrap();
            let verifying_key = signing_key.verifying_key();
            return Keypair {public: verifying_key, private: signing_key}
        }

        let signing_key = SigningKey::generate(&mut OsRng);
        let verifying_key = signing_key.verifying_key();

        // Save private key to disk as PEM format (like PGP keys)
        save_private_key(&signing_key, "node.key").unwrap();

        Keypair {public: verifying_key, private: signing_key}
    }
}

impl fmt::Display for Keypair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // We could display private key.. if we wanted to
        write!(f, "{}", base32::encode(Alphabet::Z, &self.public.to_bytes()))
    }
}