use std::fmt;

#[derive(Default, Clone)]
pub struct Subscription {
    pub id: String,
    pub url: String,
    pub is_video: bool,
    pub filter: Option<String>,
}

impl fmt::Display for Subscription {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let filter = self.filter.clone().unwrap_or_default();
        write!(f, "id = {}\nurl = {}\nfilter = {}\nis_video = {}", self.id, self.url, filter, self.is_video)
    }
}
