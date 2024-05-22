
use clap::Parser;

use rcli::{Opts, CmdExecutor};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();// 全链路追踪库，打印日志
    let opt = Opts::parse();
    opt.cmd.execute().await?;
    Ok(())
}
