use crate::store::{Store, StoreError};
use crate::models::subscription::Subscription;

#[derive(Clone)]
pub struct FeedService {
    store: Store,
}

impl FeedService {
    pub fn init(store: Store) -> Self {
        Self { store }
    }

    // TODO: Remove StoreError
    pub async fn add(&self, sub: Subscription) -> Result<(), StoreError> {
        self.store.create_subscription(sub).await
    }

    // TODO: Remove StoreError
    pub async fn delete(&self, sub: Subscription)  -> Result<(), StoreError>{
        self.store.delete_subscription(sub).await
    }
}