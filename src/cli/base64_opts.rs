use super::verify_input_file;
use crate::CmdExector;
use clap::Parser;

#[derive(Debug, Parser)]
pub enum Base64SubCommand {
  #[command(name = "encode", about = "Encode a base64 string")]
  Encode(Base64EncodeOpts),

  #[command(name = "decode", about = "Decode a base64 string")]
  Decode(Base64DecodeOpts),
}

#[derive(Debug, Parser)]
pub struct Base64EncodeOpts {
  #[arg(short, long, value_parser = verify_input_file, default_value = "-")]
  pub input: String,
}

#[derive(Debug, Parser)]
pub struct Base64DecodeOpts {
  #[arg(short, long, value_parser = verify_input_file, default_value = "-")]
  pub input: String,
}

impl CmdExector for Base64EncodeOpts {
  async fn execute(self) -> anyhow::Result<()> {
    let mut reader = crate::get_reader(&self.input)?;
    let ret = crate::process_encode(&mut reader)?;
    print!("{}", ret);
    Ok(())
  }
}

impl CmdExector for Base64DecodeOpts {
  async fn execute(self) -> anyhow::Result<()> {
    let mut reader = crate::get_reader(&self.input)?;
    let ret = crate::process_decode(&mut reader)?;
    let ret = String::from_utf8(ret)?;
    print!("{}", ret);
    Ok(())
  }
}

impl CmdExector for Base64SubCommand {
  async fn execute(self) -> anyhow::Result<()> {
    match self {
      Base64SubCommand::Encode(opts) => opts.execute().await,
      Base64SubCommand::Decode(opts) => opts.execute().await,
    }
  }
}
