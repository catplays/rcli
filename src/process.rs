use std::fs;
use csv::Reader;
use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Player {
    name: String,
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}

pub fn process_csv(input : &str, output: &str) -> Result<()>{
    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);
    // 将文件内容读取出来
    for result  in reader.deserialize() {
        let record:Player = result?;
        println!("{:?}", record);
        ret.push(record);
    }
    let json = serde_json::to_string_pretty(&ret)?;
    fs::write(output, json)?;
    Ok(())
}