use anyhow::Result;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::process::Command;
use std::path::{Path, PathBuf};

use super::models::LocalFile;

const BASE_CMD: &str = "yt-dlp -x --audio-format=mp3 --audio-quality=0 -f m4a/bestaudio --write-info-json --no-progress";

#[derive(Clone)]
pub struct YTDLLoader {}

impl YTDLLoader {
    pub async fn download(&self, url: String) -> Result<LocalFile> {
        let uuid = Uuid::new_v4().to_string();
        let cmd = format!("{} {} -o {}", BASE_CMD, url, uuid);
        let path = self.execute_command(cmd)?;
        Ok(LocalFile::new(uuid, &path)?)
    }

    pub async fn download_updates(&self, url: String, date: DateTime<Utc>, filter: Option<String>) -> Result<Vec<LocalFile>> {
        let formatted_date = format!("{}", date.format("%Y%m%d"));
        let uuid = Uuid::new_v4().to_string();
        let update_args = format!("--no-write-playlist-metafiles --playlist-end 10 --dateafter {} -P \"./{}\"", formatted_date, uuid);

        // Run command
        let mut cmd = format!("{} {}", BASE_CMD, url);
        cmd = format!("{} {}", cmd, update_args);

        if let Some(filter) = filter {
            let filter_args = format!("--match-filters title~='{}'", filter);
            cmd = format!("{} {}", cmd, filter_args);
        }

        let path = self.execute_command(cmd)?.join(&uuid);

        // Collect and parse files
        let audio_files: Vec<PathBuf> = std::fs::read_dir(path)?
            .filter_map(|entry| {
                let path = entry.unwrap().path();
                if path.is_file() && path.extension().unwrap() == "mp3" {
                    Some(path)
                } else {
                    None
                }
            })
            .collect();

        let mut local_files = vec![];
        for path in audio_files {
            let handle = tokio::spawn(async move {
                let filename = path.file_name().unwrap().to_string_lossy().to_string();
                let filepath = path.parent().unwrap().to_path_buf();
                LocalFile::new(filename, &filepath)
            });

            let result = handle.await??;
            local_files.push(result);
        }

        Ok(local_files)
    }

    fn execute_command(&self, cmd: String) -> Result<PathBuf> {
        let crate_dir = std::env::var("CARGO_MANIFEST_DIR")?;
        let path = Path::new(&crate_dir).join("storage").join("downloads");

        let status = Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .current_dir(&path)
            .status()?;

        if !status.success() {
            return Err(anyhow::Error::msg("Could not execute command"));
        }

        Ok(path)
    }
}
