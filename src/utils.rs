use std::fs::File;
use std::io::Read;

pub fn get_reader(input:&str) -> anyhow::Result<Box<dyn Read>> {
    let render:Box<dyn Read> = if input == "-" {
        // 从标准输入中读取
        Box::new(std::io::stdin())
    } else {
        // 从文件中获取
        Box::new(File::open(input)?)
    };
    Ok(render)
}