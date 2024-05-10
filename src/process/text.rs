use crate::get_reader;
use crate::process_genpass;
use crate::TextSignFormat;
use anyhow::Result;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;
use std::fs;
use std::io::Read;
use std::path::Path;

trait TextSign {
  fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>>;
}

trait TextVerify {
  fn verify(&self, reader: impl Read, sig: &[u8]) -> Result<bool>;
}

trait KeyLoader {
  fn load(path: impl AsRef<Path>) -> Result<Self>
  where
    Self: Sized;
}

trait KeyGenerator {
  fn generate() -> Result<Vec<Vec<u8>>>;
}

struct Blake3 {
  key: [u8; 32],
}

struct Ed25519Signer {
  key: SigningKey,
}

struct Ed25519Verifier {
  key: VerifyingKey,
}

pub fn process_sign(input: &str, key: &str, format: TextSignFormat) -> Result<String> {
  let mut reader = get_reader(input)?;
  let signed = match format {
    TextSignFormat::Blake3 => {
      let signer = Blake3::load(key)?;
      signer.sign(&mut reader)?
    }
    TextSignFormat::Ed25519 => {
      let signer = Ed25519Signer::load(key)?;
      signer.sign(&mut reader)?
    }
  };
  let signed = URL_SAFE_NO_PAD.encode(signed);
  Ok(signed)
}

pub fn process_verify(input: &str, key: &str, format: TextSignFormat, sig: &str) -> Result<bool> {
  let mut reader = get_reader(input)?;
  let sig = URL_SAFE_NO_PAD.decode(sig)?;
  let verified = match format {
    TextSignFormat::Blake3 => {
      let verifier = Blake3::load(key)?;
      verifier.verify(&mut reader, &sig)?
    }
    TextSignFormat::Ed25519 => {
      let verifier = Ed25519Verifier::load(key)?;
      verifier.verify(&mut reader, &sig)?
    }
  };
  Ok(verified)
}

pub fn process_generate(format: TextSignFormat) -> Result<Vec<Vec<u8>>> {
  match format {
    TextSignFormat::Blake3 => Blake3::generate(),
    TextSignFormat::Ed25519 => Ed25519Signer::generate(),
  }
}

impl TextSign for Blake3 {
  fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    let ret = blake3::keyed_hash(&self.key, &buf);
    Ok(ret.as_bytes().to_vec())
  }
}

impl TextVerify for Blake3 {
  fn verify(&self, mut reader: impl Read, sig: &[u8]) -> Result<bool> {
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    let ret = blake3::keyed_hash(&self.key, &buf);
    let hash = ret.as_bytes();
    Ok(hash == sig)
  }
}

impl KeyLoader for Blake3 {
  fn load(path: impl AsRef<Path>) -> Result<Self> {
    let key = fs::read(path)?;
    Self::try_new(key)
  }
}

impl TextSign for Ed25519Signer {
  fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    let sig = self.key.sign(&buf);
    Ok(sig.to_bytes().to_vec())
  }
}

impl TextVerify for Ed25519Verifier {
  fn verify(&self, mut reader: impl Read, sig: &[u8]) -> Result<bool> {
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    let sig = (&sig[..64]).try_into()?;
    let signature = Signature::from_bytes(sig);
    let ret = self.key.verify(&buf, &signature).is_ok();
    Ok(ret)
  }
}

impl KeyLoader for Ed25519Signer {
  fn load(path: impl AsRef<Path>) -> Result<Self> {
    let key = fs::read(path)?;
    Self::try_new(&key)
  }
}

impl KeyLoader for Ed25519Verifier {
  fn load(path: impl AsRef<Path>) -> Result<Self> {
    let key = fs::read(path)?;
    Self::try_new(&key)
  }
}

impl KeyGenerator for Blake3 {
  fn generate() -> Result<Vec<Vec<u8>>> {
    let key = process_genpass(32, true, true, true, true)?;
    println!("blake3 key generator: {}", key);
    let key = key.as_bytes().to_vec();
    Ok(vec![key])
  }
}

impl KeyGenerator for Ed25519Signer {
  fn generate() -> Result<Vec<Vec<u8>>> {
    let mut csprng = OsRng;
    let signing_key = SigningKey::generate(&mut csprng);
    let public_key = signing_key.verifying_key().to_bytes().to_vec();
    let signing_key = signing_key.to_bytes().to_vec();
    Ok(vec![signing_key, public_key])
  }
}

impl Blake3 {
  pub fn new(key: [u8; 32]) -> Self {
    Self { key }
  }

  pub fn try_new(key: impl AsRef<[u8]>) -> Result<Self> {
    let key = key.as_ref();
    let key = (&key[..32]).try_into()?;
    let signer = Self::new(key);
    Ok(signer)
  }
}

impl Ed25519Signer {
  pub fn new(key: SigningKey) -> Self {
    Self { key }
  }
  pub fn try_new(key: &[u8]) -> Result<Self> {
    let key = SigningKey::from_bytes(key.try_into()?);
    let signer = Ed25519Signer::new(key);
    Ok(signer)
  }
}

impl Ed25519Verifier {
  pub fn new(key: VerifyingKey) -> Self {
    Self { key }
  }
  pub fn try_new(key: &[u8]) -> Result<Self> {
    let key = VerifyingKey::from_bytes(key.try_into()?)?;
    let verifier = Ed25519Verifier::new(key);
    Ok(verifier)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_blake3_sign_verify() -> Result<()> {
    let blake3 = Blake3::load("fuxtures/blacke3.txt")?;
    let data = b"hello world";
    let sig = blake3.sign(&mut &data[..])?;
    assert!(blake3.verify(&data[..], &sig).unwrap());
    Ok(())
  }

  #[test]
  fn test_ed25519_sign_verify() -> Result<()> {
    let sk = Ed25519Signer::load("fuxtures/ed25519.sk")?;
    let pk = Ed25519Verifier::load("fuxtures/ed25519.pk")?;
    let data = b"hello world";
    let sig = sk.sign(&mut &data[..])?;
    assert!(pk.verify(&data[..], &sig)?);
    Ok(())
  }
}
