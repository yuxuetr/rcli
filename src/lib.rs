mod cli;
mod process;
mod utils;

pub use cli::{
  Base64DecodeOpts, Base64EncodeOpts, CsvOpts, DecryptOpts, EncryptOpts, GenPassOpts,
  HttpServeOpts, Opts, TextKeyGenerateOpts, TextSignOpts, TextVerifyOpts,
};
pub use cli::{
  Base64SubCommand, HttpSubCommand, OutputFormat, SubCommand, TextSignFormat, TextSubCommand,
};
use enum_dispatch::enum_dispatch;
pub use process::process_csv;
pub use process::process_genpass;
pub use process::process_http_serve;
pub use process::{decrypt_text, encrypt_text, process_generate, process_sign, process_verify};
pub use process::{process_decode, process_encode};
pub use utils::{get_content, get_reader};

#[allow(async_fn_in_trait)]
#[enum_dispatch]
pub trait CmdExecutor {
  async fn execute(self) -> anyhow::Result<()>;
}
