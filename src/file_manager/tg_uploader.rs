use anyhow::Result;
use regex::Regex;
use teloxide::{prelude::*, types::InputFile};

use super::models::LocalFile;

const MAX_LENGTH: usize = 500;

#[derive(Clone)]
pub struct TGUploader {
    chat_id: String,
    token: String,
    bot: Bot,
}

impl TGUploader {
    pub fn new(chat_id: String, token: String) -> Self {
        Self { 
            chat_id,
            token: token.clone(),
            bot: Bot::new(token)
        }
    }

    pub async fn upload(&self, file: &LocalFile) -> Result<String> {
        // Message with audio
        let mut message = self.bot.send_audio(
            self.chat_id.clone(),
            InputFile::file(&file.path)
        )
        .title(file.info.title.clone());

        // Thumbnail
        if let Ok(url) = url::Url::parse(&file.info.image_url) {
            let thumb = InputFile::url(url);
            message = message.thumb(thumb);
        }

        // Description
        let mut desc = file.info.description.clone();
        let url_regex = Regex::new(r"https?://\S+")?;
        desc = url_regex.replace_all(&desc, "").to_string();
        desc = desc.chars().take(MAX_LENGTH).collect();
        message = message.caption(format!("{}\n{}", file.info.title, desc));

        // Send
        let result = message.send().await?;

        // Get direct url
        let uploaded_id = &result.audio().unwrap().file.id;
        let uploaded_file = self.bot.get_file(uploaded_id.clone()).send().await?;

        // Remove file
        if let Err(err) = std::fs::remove_file(&file.path) {
            // Removing is not critical, so don't return error
            eprintln!("Can not remove file: {}", err)
        }

        Ok(format!("https://api.telegram.org/file/bot{}/{}", self.token, uploaded_file.path))
    }
}
