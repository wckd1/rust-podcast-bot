mod ytdl_loader;

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde_derive::{Deserialize};

use ytdl_loader::YTDLLoader;

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

#[derive(Clone)]
pub struct FileManager{
    loader: YTDLLoader
}

impl FileManager {
    pub fn new() -> Self {
        Self { loader: YTDLLoader{} }
    }
    
    pub async fn get(&self, url: String) -> Result<Download> {
        let file = self.loader.download(url).await?;
        
        // TODO: Upload file
        // uploadURL = fm.Uploader.Upload(ctx, file)
        let upload_url = "".to_string();

        Ok(Download { url: upload_url, info: file.info })
    }

    pub async fn check_update(&self, _url: String, _date: DateTime<Utc>, _filter: Option<String>) -> Result<Vec<Download>> {
        println!("Not implemented yet");
        Ok(vec![])
    }
}
