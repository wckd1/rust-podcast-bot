#[derive(Default, Clone)]
pub struct YouTubeItem {
    pub id: String,
    pub url: String,
    pub is_video: bool,
    pub filter: Option<String>,
}
