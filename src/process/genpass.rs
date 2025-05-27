use rand::seq::SliceRandom;
use rand::thread_rng;
extern crate zxcvbn;

const UPPER: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijkmnpqrstuvwxyz";
const NUMBER: &[u8] = b"123456789";
const SYMBOL: &[u8] = b"!@#$%&^*_";

pub fn process_genpass(
  length: u8,
  upper: bool,
  lower: bool,
  number: bool,
  symbol: bool,
) -> anyhow::Result<String> {
  let mut password = Vec::new();
  let mut rng = thread_rng();
  let mut chars = Vec::new();

  if upper {
    chars.extend_from_slice(UPPER);
    password.push(*UPPER.choose(&mut rng).expect(""));
  }
  if lower {
    chars.extend_from_slice(LOWER);
    password.push(*LOWER.choose(&mut rng).expect(""));
  }
  if number {
    chars.extend_from_slice(NUMBER);
    password.push(*NUMBER.choose(&mut rng).expect(""));
  }
  if symbol {
    chars.extend_from_slice(SYMBOL);
    password.push(*SYMBOL.choose(&mut rng).expect(""));
  }

  for _ in 0..(length - password.len() as u8) {
    let c = chars.choose(&mut rng).expect("chars won't be empty");
    password.push(*c);
  }

  let password = String::from_utf8(password)?;
  Ok(password)
}
