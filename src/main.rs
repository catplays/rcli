use clap::Parser;
use tracing_subscriber::fmt::format;
use rcli::{Opts, process_csv, SubCommand};


fn main() -> anyhow::Result<()> {
    let opt = Opts::parse();
    println!("{:?}", opt);
    match opt.cmd {
        SubCommand::Csv(opt) => {
            let output: String = if let Some(output) = opt.output {
                output.clone()
            } else {
                format!("output.{}", opt.format)
            };
            process_csv(&opt.input, output, opt.format)?
        }
    }
    Ok(())
}
