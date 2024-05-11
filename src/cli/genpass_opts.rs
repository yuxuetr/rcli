use clap::Parser;
use zxcvbn::zxcvbn;

use crate::{process_genpass, CmdExector};

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

impl CmdExector for GenPassOpts {
  async fn execute(self) -> anyhow::Result<()> {
    let ret = process_genpass(
      self.length,
      self.no_uppercase,
      self.no_lowercase,
      self.no_number,
      self.no_symbol,
    )?;
    print!("{}", ret);

    let estimate = zxcvbn(&ret, &[])?;
    eprintln!("\nPassword score: {}", estimate.score());
    Ok(())
  }
}
