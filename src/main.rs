use clap::Parser;
use rcli::{process_csv, process_genpass, Base64SubCommand, Opts, SubCommand};
use rcli::{process_decode, process_encode};

fn main() -> anyhow::Result<()> {
  let opts: Opts = Opts::parse();
  match opts.cmd {
    SubCommand::Csv(opts) => {
      let output = if let Some(output) = opts.output {
        output.clone()
      } else {
        format!("output.{}", opts.format)
      };
      process_csv(&opts.input, output, opts.format)?;
    }
    SubCommand::GenPass(opts) => process_genpass(
      opts.length,
      opts.no_uppercase,
      opts.no_lowercase,
      opts.no_number,
      opts.no_symbol,
    )?,
    SubCommand::Base64(subcmd) => match subcmd {
      Base64SubCommand::Encode(opts) => process_encode(&opts.input)?,
      Base64SubCommand::Decode(opts) => process_decode(&opts.input)?,
    },
  }
  Ok(())
}
