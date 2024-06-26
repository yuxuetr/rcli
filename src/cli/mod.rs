mod base64_opts;
mod csv_opts;
mod genpass_opts;
mod http;
mod text;

pub use self::base64_opts::{Base64DecodeOpts, Base64EncodeOpts, Base64SubCommand};
pub use self::csv_opts::{CsvOpts, OutputFormat};
pub use self::genpass_opts::GenPassOpts;
pub use self::http::{HttpServeOpts, HttpSubCommand};
pub use self::text::{
  TextKeyGenerateOpts, TextSignFormat, TextSignOpts, TextSubCommand, TextVerifyOpts,
};
use clap::Parser;
use enum_dispatch::enum_dispatch;
use std::path::{Path, PathBuf};

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author)]
pub struct Opts {
  #[command(subcommand)]
  pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExector)]
pub enum SubCommand {
  #[command(name = "csv", about = "Show CSV, or convert CSV to others")]
  Csv(CsvOpts),

  #[command(name = "genpass", about = "Generate random strength password")]
  GenPass(GenPassOpts),

  #[command(subcommand, about = "Base64 encode/decode")]
  Base64(Base64SubCommand),

  #[command(subcommand, about = "Text sign/verify")]
  Text(TextSubCommand),

  #[command(subcommand, about = "HTTP server")]
  Http(HttpSubCommand),
}

// impl CmdExector for SubCommand {
//   async fn execute(self) -> anyhow::Result<()> {
//     match self {
//       SubCommand::Csv(opts) => opts.execute().await,
//       SubCommand::GenPass(opts) => opts.execute().await,
//       SubCommand::Base64(cmd) => cmd.execute().await,
//       SubCommand::Text(cmd) => cmd.execute().await,
//       SubCommand::Http(cmd) => cmd.execute().await,
//     }
//   }
// }

pub fn verify_input_file(file_name: &str) -> Result<String, &'static str> {
  if file_name == "-" || Path::new(file_name).exists() {
    Ok(file_name.into())
  } else {
    Err("File does not exists")
  }
}

pub fn verify_path(path: &str) -> Result<PathBuf, &'static str> {
  let new_path = Path::new(path);
  if new_path.exists() && new_path.is_dir() {
    Ok(path.into())
  } else {
    Err("File does not exists")
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_verify_input_file() {
    assert_eq!(verify_input_file("-"), Ok("-".into()));
    assert_eq!(verify_input_file("*"), Err("File does not exists"));
    assert_eq!(verify_input_file("Cargo.toml"), Ok("Cargo.toml".into()));
    assert_eq!(verify_input_file("no-exist"), Err("File does not exists"));
  }
}
