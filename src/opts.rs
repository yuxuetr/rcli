use clap::Parser;
use std::fmt;
use std::path::Path;
use std::str::FromStr;

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

  #[command(name = "genpass", about = "")]
  GenPass(GenPassOpts),
}

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
  Json,
  Yaml,
  Toml,
}

#[derive(Debug, Parser)]
pub struct GenPassOpts {
  #[arg(short, long, default_value_t = 16)]
  pub length: u8,

  #[arg(long, default_value_t = true)]
  pub no_uppercase: bool,

  #[arg(long, default_value_t = true)]
  pub no_lowercase: bool,

  #[arg(long, default_value_t = true)]
  pub no_number: bool,

  #[arg(long, default_value_t = true)]
  pub no_symbol: bool,
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
  #[arg(short, long, value_parser = verify_input_file)]
  pub input: String,

  #[arg(short, long)]
  pub output: Option<String>,

  #[arg(long, value_parser = parse_format, default_value = "json")]
  pub format: OutputFormat,

  #[arg(short, long, default_value_t = ',')]
  pub delimiter: char,

  #[arg(long, default_value_t = true)]
  pub header: bool,
}

fn parse_format(format: &str) -> Result<OutputFormat, anyhow::Error> {
  format.parse::<OutputFormat>()
}

impl From<OutputFormat> for &'static str {
  fn from(format: OutputFormat) -> Self {
    match format {
      OutputFormat::Json => "json",
      OutputFormat::Yaml => "yaml",
      OutputFormat::Toml => "toml",
    }
  }
}

impl FromStr for OutputFormat {
  type Err = anyhow::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "json" => Ok(OutputFormat::Json),
      "yaml" => Ok(OutputFormat::Yaml),
      "toml" => Ok(OutputFormat::Toml),
      _ => Err(anyhow::anyhow!("Invalid format")),
    }
  }
}

impl fmt::Display for OutputFormat {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", Into::<&str>::into(*self))
  }
}

fn verify_input_file(file_name: &str) -> Result<String, &'static str> {
  if Path::new(file_name).exists() {
    Ok(file_name.into())
  } else {
    Err("File does not exists")
  }
}
