use anyhow::{Error, Result};
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FileInfo {
    pub file_path: PathBuf,
    pub file_name: String,
    pub file_size: u64,
}

impl FileInfo {
    pub fn new(file_path_str: &str) -> Result<Self, Error> {
        let file_path = PathBuf::from_str(file_path_str)?;
        let file_name = file_path
            .file_name()
            .map(|n| n.to_str().unwrap_or_default().to_string())
            .unwrap_or_default();
        let file_size = file_path.metadata()?.len();

        let file_info = FileInfo {
            file_path,
            file_name,
            file_size,
        };

        Ok(file_info)
    }
}
