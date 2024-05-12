use clap::Parser;

use rcli::{Base64SubCommand, Opts, process_csv,
           process_decode, process_encode,
           process_genpass, process_text_sign,
           process_text_verify, SubCommand,
           TextSubCommand, };

fn main() -> anyhow::Result<()> {
    let opt = Opts::parse();
    match opt.cmd {
        SubCommand::Csv(opt) => {
            let output: String = if let Some(output) = opt.output {
                output.clone()
            } else {
                format!("output.{}", opt.format)
            };
            process_csv(&opt.input, output, opt.format)?
        }
        SubCommand::GenPass(opts) => {
            process_genpass(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.number,
                opts.symbol,
            )?;
        }
        SubCommand::Base64(sub_cmd) =>{
                match sub_cmd {
                    Base64SubCommand::Encode(opt) => {
                        process_encode(&opt.input, opt.format)?
                    },
                    Base64SubCommand::Decode(opt) => {
                        process_decode(&opt.input, opt.format)?
                    }
                }
        }
        SubCommand::Signature(subcmd) => match subcmd {
            TextSubCommand::Sign(opts) => {
                let sig = process_text_sign(&opts.input, &opts.key, opts.format)?;
                println!("{}", sig);
            }
            TextSubCommand::Verify(opts) => {
                let verified = process_text_verify(&opts.input, &opts.key, opts.format, &opts.sig)?;
                println!("{}", verified);
            }
            _ => {}
        },
    }
    Ok(())
}
