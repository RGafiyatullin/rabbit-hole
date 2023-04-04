use std::path::Path;
use std::pin::pin;

use ed25519_dalek::ed25519::signature::Signature;
use ed25519_dalek::{Keypair, Signer};

use crate::cli_args::{CliArgs, TeaParty};
use crate::AnyError;

pub async fn run(tea_party: &TeaParty, _cli_args: &CliArgs) -> Result<(), AnyError> {
    let node_keypair = read_openssh_file(&tea_party.key)?;
    let node_keypair = pin!(node_keypair);
    let tea_party_url = &tea_party.url;

    let signature = node_keypair.sign("hello there!".as_bytes());

    eprintln!("Hello there: {:0x?}", signature.as_bytes());

    unimplemented!()
}

pub fn read_openssh_file(path: &str) -> Result<Keypair, AnyError> {
    let key = ssh_key::PrivateKey::read_openssh_file(Path::new(path))?;
    let key_data = key.key_data();
    let Some(keypair_ed25519) = key_data.ed25519() else {return Err(format!("Expected ed25519 [found: {:?}]", key.algorithm()).into())};

    let keypair = Keypair {
        public: ed25519_dalek::PublicKey::from_bytes(&keypair_ed25519.public.0[..])?,
        secret: ed25519_dalek::SecretKey::from_bytes(&keypair_ed25519.private.to_bytes()[..])?,
    };

    Ok(keypair)
}
