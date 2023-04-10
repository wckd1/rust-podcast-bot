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
        loop {
            self.feed_service.check_for_updates();
            sleep(Duration::from_secs(self.delay)).await;
        }
    }
}
