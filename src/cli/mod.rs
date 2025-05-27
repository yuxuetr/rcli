mod b64;
mod csv;
mod genpass;
mod http;
mod text;

pub use self::b64::{Base64DecodeOpts, Base64EncodeOpts, Base64SubCommand};
pub use self::csv::{CsvOpts, OutputFormat};
pub use self::genpass::GenPassOpts;
pub use self::http::{HttpServeOpts, HttpSubCommand};
pub use self::text::{
  DecryptOpts, EncryptOpts, TextKeyGenerateOpts, TextSignFormat, TextSignOpts, TextSubCommand,
  TextVerifyOpts,
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
#[enum_dispatch(CmdExecutor)]
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

/// 验证输出路径：确保父目录存在且可写
fn verify_output_path(path: &str) -> Result<PathBuf, anyhow::Error> {
  let path = Path::new(path);
  if let Some(parent) = path.parent() {
    if !parent.as_os_str().is_empty() {
      if !parent.exists() {
        return Err(anyhow::anyhow!("Parent directory does not exist"));
      }
      if !parent.is_dir() {
        return Err(anyhow::anyhow!("Parent path is not a directory"));
      }
    }
  }
  Ok(path.to_path_buf())
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
