
use clap::Parser;
use crate::{CmdExecutor,GenPassOpts, HttpSubCommand};
use crate::Base64SubCommand;
use crate::cli::csv::CsvOpts;
use crate::cli::TextSubCommand;

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "show csv, or convert to other format")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),
    #[command(subcommand)]
    Base64(Base64SubCommand),

    #[command(subcommand)]
    Signature(TextSubCommand),
    #[command(subcommand)]
    Http(HttpSubCommand),
}

impl CmdExecutor for SubCommand {
    async fn execute(self) -> anyhow::Result<()> {
        match self {
            SubCommand::Csv(opts) => opts.execute().await,
            SubCommand::GenPass(opts) => opts.execute().await,
            SubCommand::Base64(cmd) => cmd.execute().await,
            SubCommand::Signature(cmd) => cmd.execute().await,
            SubCommand::Http(cmd) => cmd.execute().await,
        }
    }
}