use sqlx::Error;

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
    pub fn get(&self, _url: String) -> Result<Download, Error> {
        Ok(Download::default())
    }
}
