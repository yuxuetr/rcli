use crate::TextSignFormat;
use crate::process_genpass;
use anyhow::{Result, anyhow};
use base64::{Engine, engine::general_purpose::STANDARD};
use chacha20poly1305::{
  ChaCha20Poly1305, Key, Nonce,
  aead::{Aead, KeyInit},
};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;
use std::collections::HashMap;
use std::fs;
use std::io::Read;
use std::path::Path;
use std::str;

trait TextSign {
  fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>>;
}

trait TextVerify {
  fn verify(&self, reader: &mut dyn Read, sig: &[u8]) -> Result<bool>;
}

#[allow(unused)]
trait KeyLoader {
  fn load(path: impl AsRef<Path>) -> Result<Self>
  where
    Self: Sized;
}

trait KeyGenerator {
  fn generate() -> Result<HashMap<&'static str, Vec<u8>>>;
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

pub fn process_sign(reader: &mut dyn Read, key: &[u8], format: TextSignFormat) -> Result<Vec<u8>> {
  let signer: Box<dyn TextSign> = match format {
    TextSignFormat::Blake3 => Box::new(Blake3::try_new(key)?),
    TextSignFormat::Ed25519 => Box::new(Ed25519Signer::try_new(key)?),
  };
  signer.sign(reader)
}

pub fn process_verify(
  reader: &mut dyn Read,
  key: &[u8],
  sig: &[u8],
  format: TextSignFormat,
) -> Result<bool> {
  let verifier: Box<dyn TextVerify> = match format {
    TextSignFormat::Blake3 => Box::new(Blake3::try_new(key)?),
    TextSignFormat::Ed25519 => Box::new(Ed25519Verifier::try_new(key)?),
  };
  verifier.verify(reader, sig)
}

pub fn process_generate(format: TextSignFormat) -> Result<HashMap<&'static str, Vec<u8>>> {
  match format {
    TextSignFormat::Blake3 => Blake3::generate(),
    TextSignFormat::Ed25519 => Ed25519Signer::generate(),
  }
}

// rcli text encrypt --key "xxxxxx" => 加密并输出base64
pub fn encrypt_text(plaintext: &str, key_base64: &str) -> anyhow::Result<String> {
  let key_bytes = STANDARD.decode(key_base64)?;
  if key_bytes.len() != 32 {
    return Err(anyhow!("密钥长度必须为 32 字节"));
  }
  let key = Key::from_slice(&key_bytes);
  let cipher = ChaCha20Poly1305::new(key);

  let mut nonce_bytes = [0u8; 12];
  getrandom::getrandom(&mut nonce_bytes).map_err(|e| anyhow!("生成随机数失败: {}", e))?;
  let nonce = Nonce::from_slice(&nonce_bytes);

  let ciphertext = cipher
    .encrypt(nonce, plaintext.as_bytes())
    .map_err(|e| anyhow!("加密错误: {}", e))?;

  let mut combined = nonce_bytes.to_vec();
  combined.extend(ciphertext);
  Ok(STANDARD.encode(combined))
}

// rcli text decrypt -key"XXX" >base64 > binary> 解密文本
pub fn decrypt_text(ciphertext_base64: &str, key_base64: &str) -> anyhow::Result<String> {
  let key_bytes = STANDARD.decode(key_base64)?;
  if key_bytes.len() != 32 {
    return Err(anyhow!("密钥长度必须为 32 字节"));
  }
  let key = Key::from_slice(&key_bytes);
  let cipher = ChaCha20Poly1305::new(key);

  let combined = STANDARD.decode(ciphertext_base64)?;
  if combined.len() < 12 {
    return Err(anyhow!("无效的密文格式"));
  }

  let nonce = Nonce::from_slice(&combined[..12]);
  let ciphertext = &combined[12..];

  let plaintext_bytes = cipher
    .decrypt(nonce, ciphertext)
    .map_err(|e| anyhow!("解密错误: {}", e))?;
  let plaintext = str::from_utf8(&plaintext_bytes)?;
  Ok(plaintext.to_string())
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
  fn verify(&self, reader: &mut dyn Read, sig: &[u8]) -> Result<bool> {
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
  fn verify(&self, reader: &mut dyn Read, sig: &[u8]) -> Result<bool> {
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
    Self::try_new(key)
  }
}

impl KeyLoader for Ed25519Verifier {
  fn load(path: impl AsRef<Path>) -> Result<Self> {
    let key = fs::read(path)?;
    Self::try_new(key)
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

impl KeyGenerator for Blake3 {
  fn generate() -> Result<HashMap<&'static str, Vec<u8>>> {
    let key = process_genpass(32, true, true, true, true)?;
    let mut map = HashMap::new();
    map.insert("blake3.txt", key.as_bytes().to_vec());
    Ok(map)
  }
}

impl KeyGenerator for Ed25519Signer {
  fn generate() -> Result<HashMap<&'static str, Vec<u8>>> {
    let mut csprng = OsRng;
    let sk = SigningKey::generate(&mut csprng);
    let pk: VerifyingKey = (&sk).into();
    let mut map = HashMap::new();
    map.insert("ed25519.sk", sk.to_bytes().to_vec());
    map.insert("ed25519.pk", pk.to_bytes().to_vec());
    Ok(map)
  }
}

impl Ed25519Signer {
  pub fn new(key: &[u8; 32]) -> Self {
    let key = SigningKey::from_bytes(key);
    Self { key }
  }
  pub fn try_new(key: impl AsRef<[u8]>) -> Result<Self> {
    let key = key.as_ref();
    let key = (&key[..32]).try_into()?;
    Ok(Self::new(key))
  }
}

impl Ed25519Verifier {
  pub fn try_new(key: impl AsRef<[u8]>) -> Result<Self> {
    let key = key.as_ref();
    let key = (&key[..32]).try_into()?;
    let key = VerifyingKey::from_bytes(key)?;
    Ok(Self { key })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  const KEY: &[u8] = include_bytes!("../../fuxtures/blake3.txt");

  #[test]
  fn test_blake3_sign() -> Result<()> {
    let mut reader = "hello".as_bytes();
    let mut reader1 = "hello".as_bytes();
    let format = TextSignFormat::Blake3;
    let sig = process_sign(&mut reader, KEY, format)?;
    let ret = process_verify(&mut reader1, KEY, &sig, format)?;
    assert!(ret);
    Ok(())
  }

  #[test]
  fn test_blake3_verify() -> Result<()> {
    let mut reader_for_sign = "hello".as_bytes();
    let mut reader_for_verify = "hello".as_bytes();
    let format = TextSignFormat::Blake3;
    let sig = process_sign(&mut reader_for_sign, KEY, format)?;
    let ret = process_verify(&mut reader_for_verify, KEY, &sig, format)?;
    assert!(ret);
    Ok(())
  }
}
