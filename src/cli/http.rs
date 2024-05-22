use std::path::PathBuf;

use clap::Parser;
use enum_dispatch::enum_dispatch;
use crate::{CmdExecutor, process_http_serve};

use super::verify_path;

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExecutor)]
pub enum HttpSubCommand {
    #[command(about = "A http server deal with a bound directory")]
    Serve(HttpServeOpt)
}

#[derive(Debug, Parser)]
pub struct HttpServeOpt {
    #[arg(short, long, value_parser = verify_path, default_value = ".")]
    pub dir: PathBuf,
    #[arg(short, long, default_value_t = 8080)]
    pub port: u16,
}

impl CmdExecutor for HttpServeOpt {
    async fn execute(self) -> anyhow::Result<()> {
        println!("dir:{:?}, and port:{}", self.dir, self.port);
        process_http_serve(self.dir, self.port).await?;
        Ok(())
    }
}