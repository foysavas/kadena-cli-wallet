extern crate ed25519_dalek;
extern crate hex;
extern crate rand;
extern crate serde;
extern crate serde_json;

use serde::{Deserialize, Serialize};
use serde_json::Result;

use ed25519_dalek::Keypair;
use rand::rngs::OsRng;

#[derive(Serialize, Deserialize)]
struct KeypairConfig {
  public: String,
  private: String,
}

pub fn run() -> Result<()> {
  let mut csprng = OsRng {};
  let keypair: Keypair = Keypair::generate(&mut csprng);

  let public_key_bytes: [u8; ed25519_dalek::PUBLIC_KEY_LENGTH] = keypair.public.to_bytes();
  let public_key_hex = hex::encode(public_key_bytes);

  let private_key_bytes: [u8; ed25519_dalek::SECRET_KEY_LENGTH] = keypair.secret.to_bytes();
  let private_key_hex = hex::encode(private_key_bytes);

  let keypair_config = KeypairConfig {
    public: public_key_hex.to_owned(),
    private: private_key_hex.to_owned(),
  };

  // println!("{}", public_key_hex);
  // println!("{}", private_key_hex);
  let j = serde_json::to_string(&keypair_config)?;
  println!("{}", j);

  Ok(())
}
