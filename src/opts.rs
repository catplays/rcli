use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "show csv, or convert to other format")]
    Csv(CsvOpts),
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_input_file)]
    pub input: String,
    // 输入
    #[arg(short, long, default_value = "output.json")]
    pub output: String,

    // 输出
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
    // 分隔符
    #[arg(long, default_value_t = true)]
   pub  header: bool,
}


fn verify_input_file(file_name: &str) -> Result<String, String> {
    if std::path::Path::new(file_name).exists() {
        Ok(file_name.into())
    } else {
        Err("File not exist".into())
    }
}
