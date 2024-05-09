use anyhow::Result;
use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use std::fs::File;
use std::io::Read;

pub fn process_encode(input: &str) -> Result<()> {
  let mut reader = get_reader(input)?;
  let mut buf = Vec::new();
  reader.read_to_end(&mut buf)?;
  let encoded = URL_SAFE.encode(buf);
  println!("{}", encoded);
  Ok(())
}

pub fn process_decode(input: &str) -> Result<()> {
  let mut reader = get_reader(input)?;
  let mut buf = String::new();
  reader.read_to_string(&mut buf)?;
  let buf = buf.trim();
  let decoded = URL_SAFE.decode(buf)?;
  let decoded = String::from_utf8(decoded)?;
  println!("{}", decoded);
  Ok(())
}

fn get_reader(input: &str) -> Result<Box<dyn Read>> {
  let reader: Box<dyn Read> = if input == "-" {
    Box::new(std::io::stdin())
  } else {
    Box::new(File::open(input)?)
  };
  Ok(reader)
}

#[cfg(test)]
mod tests {
  use crate::{process_decode, process_encode};

  #[test]
  fn test_process_encode() {
    let input = "Cargo.toml";
    assert!(process_encode(input).is_ok());
  }

  #[test]
  fn test_process_denoce() {
    let input = "fuxtures/b64.txt";
    assert!(process_decode(input).is_ok());
  }
}
