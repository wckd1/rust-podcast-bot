use std::time::Duration;

use tokio::time::sleep;

use crate::feed_service::FeedService;

pub struct Updater {
    delay: u64,
    feed_service: FeedService,
}

impl Updater {
    pub fn init(delay: u64, feed_service: FeedService) -> Self {
        Self { delay, feed_service }
    }

    pub async fn start(&self) {
        println!("Starting updater with {} seconds interval", self.delay);

        loop {
            if let Err(err) = self.feed_service.check_for_updates().await {
                eprintln!("Update failed: {}", err)
            };
            sleep(Duration::from_secs(self.delay)).await;
        }
    }
}
