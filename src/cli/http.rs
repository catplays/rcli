use std::path::PathBuf;

use clap::Parser;
use crate::{CmdExecutor, process_http_serve};

use super::verify_path;

#[derive(Debug, Parser)]
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

impl CmdExecutor for HttpSubCommand {
    async fn execute(self) -> anyhow::Result<()> {
        match self {
            HttpSubCommand::Serve(opt) => {
                println!("dir:{:?}, and port:{}", opt.dir, opt.port);
                process_http_serve(opt.dir, opt.port).await?;
                Ok(())
            }
        }
    }
}