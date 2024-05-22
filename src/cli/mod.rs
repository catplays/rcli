mod genpass;
mod opts;
mod base64;
mod signature;
mod http;
mod csv;

use std::path::{Path, PathBuf};


pub use self:: {
    opts::*,
    base64::*,
    csv::*,
    genpass::*,
    http::*,
    signature::*
};

fn verify_file(filename: &str) -> Result<String, &'static str> {
    // if input is "-" or file exists
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist")
    }
}


fn verify_path(path: &str) -> Result<PathBuf, &'static str> {
    // if input is "-" or file exists
    let p = Path::new(path);
    if p.exists() && p.is_dir() {
        Ok(path.into())
    } else {
        Err("Path does not exist or is not a directory")
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_file("-"), Ok("-".into()));
        assert_eq!(verify_file("*"),Err("File does not exist"));
        assert_eq!(verify_file("Cargo.toml"),Ok("Cargo.toml".into()));
    }
}