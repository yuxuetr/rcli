use anyhow::Result;
use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use std::io::Read;

pub fn process_encode(reader: &mut dyn Read) -> Result<String> {
  let mut buf = Vec::new();
  reader.read_to_end(&mut buf)?;
  let encoded = URL_SAFE.encode(buf);
  Ok(encoded)
}

pub fn process_decode(reader: &mut dyn Read) -> Result<Vec<u8>> {
  let mut buf = String::new();
  reader.read_to_string(&mut buf)?;
  let buf = buf.trim();
  let decoded = URL_SAFE.decode(buf)?;
  Ok(decoded)
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::get_reader;

  #[test]
  fn test_process_encode() -> Result<()> {
    let input = "Cargo.toml";
    let mut reader = get_reader(input)?;
    assert!(process_encode(&mut reader).is_ok());
    Ok(())
  }

  #[test]
  fn test_process_decode() -> Result<()> {
    let input = "fuxtures/b64.txt";
    let mut reader = get_reader(input)?;
    assert!(process_decode(&mut reader).is_ok());
    Ok(())
  }
}
