#[macro_use]
extern crate log;

use std::env;
use dotenv::dotenv;

mod models;
mod store;
mod feed_service;
mod bot;

use store::Store;
use feed_service::FeedService;
use bot::BotSerivce;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    // FIXME: Logs are not visible in terminal
    info!("Starting bot service");

    let store = Store::init().await.expect("Database can not be inited");

    let feed_service = FeedService::init(store);

    let token = env::var("BOT_TOKEN").expect("Bot token not set");
    let bot = BotSerivce::init(token, feed_service);

    // TODO: Add grasefull shutdown
    // TODO: Run in separate thread
    bot.start().await;
}
