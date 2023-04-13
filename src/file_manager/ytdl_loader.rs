use anyhow::Result;
use uuid::Uuid;
use std::process::Command;
use std::path::{Path, PathBuf};

use super::FileInfo;

const BASE_CMD: &str = "yt-dlp -x --audio-format=mp3 --audio-quality=0 -f m4a/bestaudio --write-info-json --no-progress";

pub struct LocalFile {
	// path: String,
	pub info: FileInfo
}

impl LocalFile {
    fn new(id: String, dir: &PathBuf) -> Result<Self> {
        // let file_path = Path::new(dir).join(format!("{}.mp3", id));
        let info_path = Path::new(dir).join(format!("{}.tmp.info.json", id));

        Ok(
            Self { 
                // path: file_path, 
                info: parse_info(info_path)?
            }
        )
    }
}

#[derive(Clone)]
pub struct YTDLLoader {}

impl YTDLLoader {
    pub async fn download(&self, url: String) -> Result<LocalFile> {
        let uuid = Uuid::new_v4().to_string();
        let cmd = format!("{} {} -o {}.tmp", BASE_CMD, url, uuid);
        let crate_dir = std::env::var("CARGO_MANIFEST_DIR")?;
        let path = Path::new(&crate_dir).join("storage").join("downloads");

        let status = Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .current_dir(&path)
            .status()?;

        if !status.success() {
            return Err(anyhow::Error::msg(format!("Could not download {}", url)));
        }
        
        Ok(LocalFile::new(uuid, &path)?)
    }
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
