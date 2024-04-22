use std::fs;
use csv::Reader;
use serde::{Deserialize, Serialize};
use anyhow::Result;
use crate::opts::OutputFormat;

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

pub fn process_csv(input : &str, output: String, format: OutputFormat) -> Result<()>{
    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);
    let headers = reader.headers()?.clone();
    // 将文件内容读取出来
    for result  in reader.records() {
        let record = result?;
        println!("{:?}", record);

        // headers.iter -> 使用headers的迭代器 第一行的多列,即key名
        // record.iter -> 使用了record的迭代器 每行的多列 即value
        // zip -> 将两个迭代器合并为一个元组的迭代器[(header, record),...]
        // collect::<Value> -> 将元组的迭代器转换为JSON_Value
        let json_str = headers.iter()
            .zip(record.iter())
            .collect::<serde_json::Value>();

        ret.push(json_str);
    }
    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&ret)?,
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
    };
    fs::write(output, content)?;
    Ok(())
}
