use chrono::{DateTime, Utc};

#[derive(Clone)]
pub struct Subscription {
    pub id: String,
    pub url: String,
    pub filter: Option<String>,
    pub update_interval: u8,
    pub last_updated: DateTime<Utc>,
}
