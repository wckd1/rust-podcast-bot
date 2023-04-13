use anyhow::Result;
use uuid::Uuid;
use std::process::Command;
use std::path::Path;

use super::models::LocalFile;

const BASE_CMD: &str = "yt-dlp -x --audio-format=mp3 --audio-quality=0 -f m4a/bestaudio --write-info-json --no-progress";

#[derive(Clone)]
pub struct YTDLLoader {}

impl YTDLLoader {
    pub async fn download(&self, url: String) -> Result<LocalFile> {
        let uuid = Uuid::new_v4().to_string();
        let cmd = format!("{} {} -o {}", BASE_CMD, url, uuid);
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
