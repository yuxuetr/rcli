use clap::Parser;
use std::path::Path;

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
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
  #[arg(short, long, value_parser = verify_input_file)]
  pub input: String,

  #[arg(short, long, default_value = "output.json")]
  pub output: String,

  #[arg(short, long, default_value_t = ',')]
  pub delimiter: char,

  #[arg(long, default_value_t = true)]
  pub header: bool,
}

fn verify_input_file(file_name: &str) -> Result<String, &'static str> {
  if Path::new(file_name).exists() {
    Ok(file_name.into())
  } else {
    Err("File does not exists")
  }
}
