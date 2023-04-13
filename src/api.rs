use tide::Response;

use crate::{feed_service::FeedService, models::rss_feed::RSSFeed};

#[derive(Clone)]
pub struct APIState {
    rss_key: String,
    feed_service: FeedService
}

impl APIState {
    pub fn new(rss_key: String, feed_service: FeedService) -> Self {
        Self { rss_key, feed_service }
    }
}

pub async fn start_api(state: APIState) -> tide::Result<()> {
    let mut app = tide::with_state(state);

    // Middlewares
    app.with(tide::log::LogMiddleware::new());

    // Routes
    app.at("/rss/:secret/").get(get_rss);

    // Start service
    println!("Starting api at 8080");
    app.listen("0.0.0.0:8080").await?;
    Ok(())
}

async fn get_rss(req: tide::Request<APIState>) -> tide::Result {
    let secret = req.param("secret")?.to_string();
    if secret != req.state().rss_key {
        return Ok(Response::builder(tide::StatusCode::Forbidden)
            .body("Not valid RSS key")
            .build()
        )
    }

    let episodes = req.state().feed_service.get_episodes(20).await?;
    let feed = RSSFeed::init(episodes).build().to_string();
    let body = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n".to_owned() + &feed;

    let response = Response::builder(tide::StatusCode::Ok)
        .body(body.to_string())
        .header("Content-Type", "application/xml; charset=UTF-8")
        .build();

    Ok(response)
}
