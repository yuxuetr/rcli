use super::{verify_input_file, verify_path};
use crate::{get_content, get_reader, process_generate, process_sign, process_verify, CmdExector};
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
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
  pub output_dir: PathBuf,
}

impl CmdExector for TextSubCommand {
  async fn execute(self) -> anyhow::Result<()> {
    match self {
      TextSubCommand::Sign(opts) => opts.execute().await,
      TextSubCommand::Verify(opts) => opts.execute().await,
      TextSubCommand::Generate(opts) => opts.execute().await,
    }
  }
}

impl CmdExector for TextSignOpts {
  async fn execute(self) -> anyhow::Result<()> {
    let mut reader = get_reader(&self.input)?;
    let key = get_content(&self.key)?;
    let sig = process_sign(&mut reader, &key, self.format)?;
    let encoded = URL_SAFE_NO_PAD.encode(sig);
    print!("{}", encoded);
    Ok(())
  }
}

impl CmdExector for TextVerifyOpts {
  async fn execute(self) -> anyhow::Result<()> {
    let mut reader = get_reader(&self.input)?;
    let key = get_content(&self.key)?;
    let decoded = URL_SAFE_NO_PAD.decode(&self.sig)?;
    let verified = process_verify(&mut reader, &key, &decoded, self.format)?;
    if verified {
      println!("Signature verified");
    } else {
      println!("Signature not verified");
    }
    Ok(())
  }
}

impl CmdExector for TextKeyGenerateOpts {
  async fn execute(self) -> anyhow::Result<()> {
    let key = process_generate(self.format)?;
    for (k, v) in key {
      tokio::fs::write(self.output_dir.join(k), v).await?;
    }
    Ok(())
  }
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
