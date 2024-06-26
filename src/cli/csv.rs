use std::fmt;
use std::str::FromStr;
use clap::Parser;
use crate::{CmdExecutor, process_csv};

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_input_file)]
    pub input: String,
    // 输入
    #[arg(short, long)]
    pub output: Option<String>,

    #[arg(long, value_parser = parse_format, default_value = "json")]
    pub format: OutputFormat,

    // 输出
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
    // 分隔符
    #[arg(long, default_value_t = true)]
    pub header: bool,
}



impl CmdExecutor for CsvOpts{
    async fn execute(self) -> anyhow::Result<()> {
        let output: String = if let Some(output) = self.output {
            output.clone()
        } else {
            format!("output.{}", self.format)
        };
        process_csv(&self.input, output, self.format)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
}




fn verify_input_file(file_name: &str) -> Result<String, String> {
    if std::path::Path::new(file_name).exists() {
        Ok(file_name.into())
    } else {
        Err("File not exist".into())
    }
}

fn parse_format(format: &str) -> Result<OutputFormat, anyhow::Error> {
    format.parse()
}

// 实现from trait， 可将OutputFormat枚举，通过into方法转为str
impl From<OutputFormat> for &'static str {
    fn from(format: OutputFormat) -> Self {
        match format {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
        }
    }
}

// 实现FromStr， 可将string转为enum
impl FromStr for OutputFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            _ => Err(anyhow::anyhow!("Invalid format")),
        }
    }
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
