use super::verify_input_file;
use crate::CmdExecutor;
use clap::Parser;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
  Json,
  Yaml,
  Toml,
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

impl CmdExecutor for CsvOpts {
  async fn execute(self) -> anyhow::Result<()> {
    let output = if let Some(output) = self.output {
      output
    } else {
      format!("output.{}", self.format)
    };
    crate::process_csv(&self.input, output, self.format)
  }
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
