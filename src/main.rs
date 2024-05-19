use std::fs;
use clap::Parser;

use rcli::{Base64SubCommand, Opts, process_csv, process_decode, process_encode, process_genpass, process_text_sign, process_text_verify, process_text_generate, SubCommand, TextSubCommand, HttpSubCommand, process_http_serve};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();// 全链路追踪库，打印日志
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
            let pass = process_genpass(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.number,
                opts.symbol,
            )?;
            println!("{}", pass);
        }
        SubCommand::Base64(sub_cmd) =>{
                match sub_cmd {
                    Base64SubCommand::Encode(opt) => {
                        let encoded = process_encode(&opt.input, opt.format)?;
                        println!("{}", encoded)
                    },
                    Base64SubCommand::Decode(opt) => {
                        let decoded = process_decode(&opt.input, opt.format)?;
                        println!("{}", decoded)
                    }
                }
        }
        SubCommand::Signature(sub_cmd) => match sub_cmd {
            TextSubCommand::Sign(opts) => {
                let sig = process_text_sign(&opts.input, &opts.key, opts.format)?;
                println!("{}", sig);
            }
            TextSubCommand::Verify(opts) => {
                let verified = process_text_verify(&opts.input, &opts.key, opts.format, &opts.sig)?;
                println!("{}", verified);
            }
            TextSubCommand::Generate(opts) => {
                let key = process_text_generate(opts.format)?;
                match opts.format {
                    rcli::TextSignFormat::Blake3 => {
                        let name = opts.output.join("blake3.txt");
                        fs::write(name, &key[0])?;
                    }
                    rcli::TextSignFormat::Ed25519 => {
                        let name = &opts.output;
                        fs::write(name.join("ed25519.sk"), &key[0])?;
                        fs::write(name.join("ed25519.pk"), &key[1])?;
                    }
                }
            }
        },
        SubCommand::Http(cmd) => match cmd {
            HttpSubCommand::Serve(opt) => {
                println!("dir:{:?}, and port:{}", opt.dir, opt.port);
                process_http_serve(opt.dir, opt.port).await?
            }
        }
    }
    Ok(())
}
