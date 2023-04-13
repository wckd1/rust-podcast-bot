use anyhow::Result;
use uuid::Uuid;
use chrono::{Utc, DateTime};
use tokio::sync::Semaphore;
use std::sync::Arc;

use crate::file_manager::{FileManager, models::Download};
use crate::store::Store;
use crate::models::youtube_item::YouTubeItem;
use crate::models::subscription::Subscription;
use crate::models::episode::{Episode, Enclosure};

#[derive(Clone)]
pub struct FeedService {
    store: Store,
    file_manager: FileManager,
}

impl FeedService {
    pub fn new(store: Store, file_manager: FileManager) -> Self {
        Self { store, file_manager }
    }
    
    // Public
    pub async fn add(&self, item: YouTubeItem) -> Result<()> {
        match item.is_video {
            true => {
                let download = self.file_manager.get(item.url.clone()).await?;
                self.add_episode(download).await
            }
            false => {
                let sub = Subscription { 
                    id: item.id, 
                    url: item.url, 
                    filter: item.filter,
                    update_interval: 24,
                    last_updated: Utc::now()
                };
                self.store.create_subscription(sub).await
            }
        }
    }

    pub async fn delete(&self, item: YouTubeItem) -> Result<()> {
        self.store.delete_subscription(item.id).await
    }

    pub async fn check_for_updates(&self) -> Result<()> {
        let now = Utc::now();

        // Get only subscriptions that required to be updated
        let pending: Vec<Subscription> = self.store.get_subscriptions()
            .await?
            .into_iter()
            .filter(|sub| (sub.last_updated < now) || (sub.last_updated == now))
            .collect();
        
        // No updates are required
        if pending.len() == 0 {
            println!("No updates are required");
            return Ok(())
        }

        // Run update tasks in parallel with limit of 3 at a time
        let sem = Arc::new(Semaphore::new(3));
        for sub in pending {
            let permit = Arc::clone(&sem).acquire_owned().await;
            let fs = Arc::new(self.clone());

            tokio::spawn(async move {
                let _permit = permit; // Take permit until task is finished
                if let Err(err) = fs.update_subscription(now, sub).await {
                    eprintln!("Can not update subscription: {}", err);
                }
            });
        }

        Ok(())
    }

    pub async fn get_episodes(&self, limit: u64) -> Result<Vec<Episode>> {
        self.store.get_episodes(limit).await
    }

    // Private
    async fn add_episode(&self, download: Download) -> Result<()> {
        let episode = Episode {
            uuid: Uuid::new_v4().to_string(),
            enclosure: Enclosure {
                url: download.url,
                length: download.info.length,
                enclosure_type: download.info.file_type,
            },
            link: download.info.link,
            image: download.info.image_url,
            title: download.info.title,
            description: download.info.description,
            author: download.info.author,
            duration: download.info.duration,
            pub_date: download.info.date,
        };

        self.store.create_episode(episode).await
    }

    async fn update_subscription(&self, update_time: DateTime<Utc>, mut sub: Subscription) -> Result<()> {
        let downloads = self.file_manager.check_update(
            sub.url.clone(),
            sub.last_updated.clone(),
            sub.filter.clone()
        )
        .await?;

        for dl in downloads {
            self.add_episode(dl).await?
        }

        sub.last_updated = update_time;
        self.store.update_subscription(sub).await?;

        Ok(())
    }
}
