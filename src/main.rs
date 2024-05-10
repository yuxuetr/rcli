use std::fs;

use clap::Parser;
use rcli::{
  process_csv, process_decode, process_encode, process_generate, process_genpass, process_sign,
  process_verify,
};
// process_http_serve,
// HttpSubCommand,
use rcli::{Base64SubCommand, Opts, SubCommand, TextSignFormat, TextSubCommand};
use zxcvbn::zxcvbn;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  tracing_subscriber::fmt::init();
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
    SubCommand::GenPass(opts) => {
      let password = process_genpass(
        opts.length,
        opts.no_uppercase,
        opts.no_lowercase,
        opts.no_number,
        opts.no_symbol,
      )?;
      print!("{}", password);
      let estimate = zxcvbn(&password, &[])?;
      println!("\npassword score: {}", estimate.score());
    }
    SubCommand::Base64(subcmd) => match subcmd {
      Base64SubCommand::Encode(opts) => {
        let encoded = process_encode(&opts.input)?;
        print!("{}", encoded);
      }
      Base64SubCommand::Decode(opts) => {
        let decoded = process_decode(&opts.input)?;
        let decoded = String::from_utf8(decoded)?;
        print!("{}", decoded);
      }
    },
    SubCommand::Text(cmd) => match cmd {
      TextSubCommand::Sign(opts) => {
        let signed = process_sign(&opts.input, &opts.key, opts.format)?;
        print!("{}", signed);
      }
      TextSubCommand::Verify(opts) => {
        let verified = process_verify(&opts.input, &opts.key, opts.format, &opts.sig)?;
        println!("{}", verified);
      }
      TextSubCommand::Generate(opts) => {
        let key = process_generate(opts.format)?;
        match opts.format {
          TextSignFormat::Blake3 => {
            let name = opts.output.join("blake3.txt");
            fs::write(name, &key[0])?;
          }
          TextSignFormat::Ed25519 => {
            let name = &opts.output;
            fs::write(name.join("ed25519.sk"), &key[0])?;
            fs::write(name.join("ed25519.pk"), &key[1])?;
          }
        }
      }
    },
    // SubCommand::Http(cmd) => match cmd {
    //   HttpSubCommand::Serve(opts) => {
    //     process_http_serve(opts.dir, opts.port).await?;
    //   }
    // },
  }
  Ok(())
}
