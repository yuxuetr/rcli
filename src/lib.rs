mod cli;
mod process;
mod utils;

pub use cli::{
  Base64SubCommand, GenPassOpts, HttpSubCommand, Opts, OutputFormat, SubCommand, TextSignFormat,
  TextSubCommand,
};
pub use process::process_csv;
pub use process::process_genpass;
pub use process::process_http_serve;
pub use process::{process_decode, process_encode};
pub use process::{process_generate, process_sign, process_verify};
pub use utils::{get_content, get_reader};

#[allow(async_fn_in_trait)]
pub trait CmdExector {
  async fn execute(self) -> anyhow::Result<()>;
}
