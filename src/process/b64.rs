use std::io::Read;

use base64::engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD};
use base64::prelude::*;

use crate::Base64Format;
use crate::utils::get_reader;

pub fn process_encode(input: &str, format: Base64Format) -> anyhow::Result<String> {
    let mut rander = get_reader(input)?;
    let mut buf = Vec::new();
    rander.read_to_end(&mut buf)?;

    let encode = match format {
        Base64Format::Standard => STANDARD.encode(&buf),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(&buf)
    };

    Ok(encode)
}


pub fn process_decode(input: &str, format: Base64Format) -> anyhow::Result<String> {
    let mut rander = get_reader(input)?;
    let mut buf = String::new();
    rander.read_to_string(&mut buf)?;

    let buf = buf.trim();
    let decode = match format {
        Base64Format::Standard => STANDARD.decode(&buf)?,
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(&buf)?
    };
    let decoded = String::from_utf8(decode)?;
    Ok(decoded)
}
