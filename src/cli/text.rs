use super::{verify_input_file, verify_path};
use clap::Parser;
use std::fmt;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug, Parser)]
pub enum TextSubCommand {
  #[command(about = "Sign a message with a private/public key")]
  Sign(TextSignOpts),

  #[command(about = "Verify a signed message")]
  Verify(TextVerifyOpts),

  #[command(about = "Generate a new key")]
  Generate(TextKeyGenerateOpts),
}

#[derive(Debug, Parser)]
pub struct TextSignOpts {
  #[arg(short, long, value_parser = verify_input_file, default_value="-")]
  pub input: String,

  #[arg(short, long, value_parser = verify_input_file)]
  pub key: String,

  #[arg(long, value_parser=parse_format, default_value="blake3")]
  pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct TextVerifyOpts {
  #[arg(short, long, value_parser = verify_input_file, default_value="-")]
  pub input: String,

  #[arg(short, long, value_parser = verify_input_file)]
  pub key: String,

  #[arg(long, value_parser=parse_format, default_value="blake3")]
  pub format: TextSignFormat,

  #[arg(short, long)]
  pub sig: String,
}

#[derive(Debug, Parser)]
pub struct TextKeyGenerateOpts {
  #[arg(short, long, default_value="blake3", value_parser=parse_format)]
  pub format: TextSignFormat,

  #[arg(short, long, value_parser=verify_path)]
  pub output: PathBuf,
}

#[derive(Debug, Clone, Copy)]
pub enum TextSignFormat {
  Blake3,
  Ed25519,
}

fn parse_format(format: &str) -> Result<TextSignFormat, anyhow::Error> {
  format.parse()
}

impl FromStr for TextSignFormat {
  type Err = anyhow::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "blake3" => Ok(TextSignFormat::Blake3),
      "ed25519" => Ok(TextSignFormat::Ed25519),
      _ => Err(anyhow::anyhow!("Invalid format")),
    }
  }
}

impl From<TextSignFormat> for &'static str {
  fn from(format: TextSignFormat) -> Self {
    match format {
      TextSignFormat::Blake3 => "blake3",
      TextSignFormat::Ed25519 => "ed25519",
    }
  }
}

impl fmt::Display for TextSignFormat {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", Into::<&'static str>::into(*self))
  }
}
