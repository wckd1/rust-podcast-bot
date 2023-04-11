use anyhow::Result;
use chrono::{DateTime, Utc};

#[derive(Clone)]
pub struct FileManager{}

#[derive(Default)]
pub struct Download {
	pub url: String,
	pub info: FileInfo,
}

#[derive(Default)]
pub struct FileInfo {
    pub link: String,
    pub title: String,
    pub description: String,
    pub image_url: String,
    pub author: String,
    pub length: i8,
    pub duration: i8,
    pub date: String,
}

impl FileManager {
    pub fn get(&self, _url: String) -> Result<Download> {
        println!("Not implemented yet");
        Ok(Download::default())
    }

    pub async fn check_update(&self, _url: String, _date: DateTime<Utc>, _filter: Option<String>) -> Result<Vec<Download>> {
        println!("Not implemented yet");
        Ok(vec![])
    }
}
