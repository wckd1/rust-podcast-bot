use anyhow::Result;
use serde_derive::Deserialize;
use std::path::{Path, PathBuf};

pub struct LocalFile {
	pub path: PathBuf,
	pub info: FileInfo
}

impl LocalFile {
    pub fn new(id: String, dir: &PathBuf) -> Result<Self> {
        let file_path = Path::new(dir).join(format!("{}.mp3", id));
        let info_path = Path::new(dir).join(format!("{}.info.json", id));

        Ok(
            Self { 
                path: file_path, 
                info: LocalFile::parse_info(info_path)?
            }
        )
    }

    fn parse_info(path: PathBuf) -> Result<FileInfo> {
        // Get content of file
        let content = std::fs::read_to_string(&path)?;
        // Parse to FileInfo structure
        let mut info = serde_json::from_str::<FileInfo>(&content)?;
        // Remove file
        if let Err(err) = std::fs::remove_file(path) {
            // Removing is not critical, so don't return error
            eprintln!("Can not remove file: {}", err)
        }
    
        info.file_type = "audio/mpeg".to_string();
    
        Ok(info)
    }
}

pub struct Download {
	pub url: String,
	pub info: FileInfo,
}

#[derive(Deserialize)]
pub struct FileInfo {
    #[serde(skip)]
    pub file_type: String,
    #[serde(rename = "webpage_url")]
    pub link: String,
    pub title: String,
    pub description: String,
    #[serde(rename = "thumbnail")]
    pub image_url: String,
    #[serde(rename = "uploader")]
    pub author: String,
    #[serde(rename = "filesize")]
    pub length: u64,
    pub duration: u64,
    #[serde(rename = "upload_date")]
    pub date: String,
}
