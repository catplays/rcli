use clap::Parser;
use rcli::{Opts, process_csv, SubCommand};


fn main() -> anyhow::Result<()> {
    let opt = Opts::parse();
    println!("{:?}", opt);
    match opt.cmd {
        SubCommand::Csv(opt) => {
            process_csv(&opt.input, &opt.output)?
        }
    }
    Ok(())
}
