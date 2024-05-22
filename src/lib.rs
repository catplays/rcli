mod process;
mod cli;
mod utils;

pub use cli::{Opts, SubCommand};
pub use cli::{GenPassOpts};
pub use cli::{Base64Format, Base64SubCommand, TextSubCommand, TextSignFormat};
pub use cli::{HttpSubCommand};
pub use process::*;

#[allow(async_fn_in_trait)]
pub trait CmdExecutor {
    async fn execute(self) -> anyhow::Result<()>;
}