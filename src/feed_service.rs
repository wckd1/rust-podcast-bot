use anyhow::Result;
use uuid::Uuid;

use crate::file_manager::FileManager;
use crate::store::Store;
use crate::models::subscription::Subscription;
use crate::models::episode::{Episode, Enclosure};

#[derive(Clone)]
pub struct FeedService {
    store: Store,
    file_manager: FileManager,
}

impl FeedService {
    pub fn init(store: Store, file_manager: FileManager) -> Self {
        Self { store, file_manager }
    }
    
    pub async fn add(&self, sub: Subscription) -> Result<()> {
        match sub.is_video {
            true => {
                let download = self.file_manager.get(sub.url.clone())?;

                let episode = Episode {
                    uuid: Uuid::new_v4().to_string(),
                    enclosure: Enclosure {
                        url: download.url,
                        length: download.info.length,
                        enclosure_type: "audio/mpeg".to_string(),
                    },
                    link: download.info.link,
                    image: download.info.image_url,
                    title: download.info.title,
                    description: "<![CDATA[".to_string() + &download.info.description + "]]>",
                    author: download.info.author,
                    duration: download.info.duration,
                    pub_date: download.info.date,
                };

                self.store.create_episode(episode).await
            },
            false => self.store.create_subscription(sub).await
        }
    }

    // TODO: Remove StoreError
    pub async fn delete(&self, sub: Subscription)  -> Result<()>{
        self.store.delete_subscription(sub).await
    }
}