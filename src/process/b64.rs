use std::fs::File;
use std::io::Read;
use base64::engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD};
use base64::prelude::*;
use crate::Base64Format;

pub fn process_encode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    let mut rander: Box<dyn Read> = if input == "-" {
        // 从标准输入中读取
        Box::new(std::io::stdin())
    } else {
        // 从文件中获取
        Box::new(File::open(input)?)
    };
    let mut buf = Vec::new();
    rander.read_to_end(&mut buf)?;

    let encode = match format {
        Base64Format::Standard => STANDARD.encode(&buf),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(&buf)
    };
    println!("{}", encode);
    Ok(())
}


pub fn process_decode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    let mut rander = get_render(input)?;
    let mut buf = String::new();
    rander.read_to_string(&mut buf)?;

    let buf = buf.trim();
    let decode = match format {
        Base64Format::Standard => STANDARD.decode(&buf)?,
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(&buf)?
    };
    let decoded = String::from_utf8(decode)?;
    println!("{:?}", decoded);
    Ok(())
}
fn get_render(input:&str) -> anyhow::Result<Box<dyn Read>> {
    let render:Box<dyn Read> = if input == "-" {
        // 从标准输入中读取
        Box::new(std::io::stdin())
    } else {
        // 从文件中获取
        Box::new(File::open(input)?)
    };
    Ok(render)
}