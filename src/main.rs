use clap::Parser;
use j1::{CmdExecutor, Opts};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  tracing_subscriber::fmt::init();
  let opts: Opts = Opts::parse();
  opts.cmd.execute().await?;
  Ok(())
}
