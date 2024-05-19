use std::path::PathBuf;

use clap::Parser;

use super::verify_path;

#[derive(Debug,Parser)]
pub enum HttpSubCommand {
    #[command(about="A http server deal with a bound directory")]
    Serve(HttpServeOpt)
}

#[derive(Debug,Parser)]
pub struct HttpServeOpt {
    #[arg(short, long, value_parser= verify_path, default_value=".")]
    pub dir:PathBuf,
    #[arg(short,long, default_value_t=8080)]
    pub port:u16,
}