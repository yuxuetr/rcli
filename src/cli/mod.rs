mod base64_opts;
mod csv_opts;
mod genpass_opts;

pub use self::base64_opts::Base64SubCommand;
pub use self::csv_opts::CsvOpts;
pub use self::genpass_opts::GenPassOpts;
use clap::Parser;
use std::path::Path;

pub use self::csv_opts::OutputFormat;

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author)]
pub struct Opts {
  #[command(subcommand)]
  pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
  #[command(name = "csv", about = "Show CSV, or convert CSV to others")]
  Csv(CsvOpts),

  #[command(name = "genpass", about = "Generate random strength password")]
  GenPass(GenPassOpts),

  #[command(subcommand)]
  Base64(Base64SubCommand),
}

pub fn verify_input_file(file_name: &str) -> Result<String, &'static str> {
  if file_name == "-" || Path::new(file_name).exists() {
    Ok(file_name.into())
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
