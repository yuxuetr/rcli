use super::verify_path;
use crate::CmdExecutor;
use crate::process_http_serve;
use clap::Parser;
use enum_dispatch::enum_dispatch;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExecutor)]
pub enum HttpSubCommand {
  #[command(about = "Serve a directory over HTTP")]
  Serve(HttpServeOpts),
}

#[derive(Debug, Parser)]
pub struct HttpServeOpts {
  #[arg(short, long, value_parser=verify_path, default_value=".")]
  pub dir: PathBuf,

  #[arg(long, default_value_t = 8009)]
  pub port: u16,
}

// impl CmdExecutor for HttpSubCommand {
//   async fn execute(self) -> anyhow::Result<()> {
//     match self {
//       HttpSubCommand::Serve(opts) => opts.execute().await,
//     }
//   }
// }

impl CmdExecutor for HttpServeOpts {
  async fn execute(self) -> anyhow::Result<()> {
    process_http_serve(self.dir, self.port).await
  }
}
