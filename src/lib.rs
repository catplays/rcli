mod process;
mod cli;
mod utils;

use enum_dispatch::enum_dispatch;
pub use cli::*;
pub use process::*;

#[allow(async_fn_in_trait)]
#[enum_dispatch]
pub trait CmdExecutor {
    async fn execute(self) -> anyhow::Result<()>;
}