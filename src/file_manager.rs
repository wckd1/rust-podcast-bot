pub mod models;
pub mod ytdl_loader;
pub mod tg_uploader;

use anyhow::Result;
use chrono::{DateTime, Utc};

use self::models::Download;
use self::ytdl_loader::YTDLLoader;
use self::tg_uploader::TGUploader;

#[derive(Clone)]
pub struct FileManager {
    downloader: YTDLLoader,
    uploader: TGUploader,
}

impl FileManager {
    pub fn new(downloader: YTDLLoader, uploader: TGUploader) -> Self {
        Self { downloader, uploader }
    }
    
    pub async fn get(&self, url: String) -> Result<Download> {
        let file = self.downloader.download(url).await?;
        let upload_url = self.uploader.upload(&file).await?;

        Ok(Download { url: upload_url, info: file.info })
    }

    pub async fn check_update(&self, _url: String, _date: DateTime<Utc>, _filter: Option<String>) -> Result<Vec<Download>> {
        // TODO: Add implementation
        println!("Not implemented yet");
        Ok(vec![])
    }
}
