pub mod models;
pub mod ytdl_loader;
pub mod tg_uploader;

use std::sync::Arc;
use anyhow::Result;
use chrono::{DateTime, Utc};
use tokio::sync::Semaphore;

use self::models::{Download, LocalFile};
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
        self.upload(file).await
    }

    pub async fn check_update(&self, url: String, date: DateTime<Utc>, filter: Option<String>) -> Result<Vec<Download>> {
        let files = self.downloader.download_updates(url, date, filter).await?;

        // Run update tasks in parallel with limit of 3 at a time
        let mut downloads = vec![];
        let sem = Arc::new(Semaphore::new(3));
        for file in files {
            let permit = Arc::clone(&sem).acquire_owned().await;
            let fm = Arc::new(self.clone());

            let handle = tokio::spawn(async move {
                let _permit = permit; // Take permit until task is finished
                fm.upload(file).await
            });

            let result = handle.await??;
            downloads.push(result);
        }
        
        Ok(downloads)
    }

    async fn upload(&self, file: LocalFile) -> Result<Download> {
        let upload_url = self.uploader.upload(&file).await?;
        Ok(Download { url: upload_url, info: file.info })
    }
}
