mod models;
mod store;
mod file_manager;
mod feed_service;
mod bot;
mod updater;
mod api;

use std::env;
use dotenv::dotenv;
use tokio::signal;

use store::Store;
use file_manager::{FileManager, ytdl_loader::YTDLLoader, tg_uploader::TGUploader};
use feed_service::FeedService;
use bot::BotSerivce;
use updater::Updater;
use api::{APIState, start_api};

#[tokio::main]
async fn main() {
    dotenv().ok();
    println!("Starting services");

    // Check for required parameters
    let update_interval = env::var("UPDATE_INTERVAL").unwrap().parse::<u64>().expect("Update Interval not set");
    let token = env::var("BOT_TOKEN").expect("Bot token not set");
    let chat_id = env::var("CHAT_ID").expect("Uploader chat id not set");
    let rss_key = env::var("RSS_KEY").expect("RSS key not set");

    // DB
    let store = Store::new().await.expect("Database can not be inited");
    let downloader = YTDLLoader::new();
    let uploader = TGUploader::new(chat_id, token.clone());
    let file_manager = FileManager::new(downloader, uploader);
    let feed_service = FeedService::new(store, file_manager);

    // Handlers
    let updater = Updater::new(update_interval * 60, feed_service.clone());
    let bot = BotSerivce::new(token, feed_service.clone());
    let api_state = APIState::new(rss_key, feed_service);

    // Start handlers
    let updater_task = tokio::spawn(async move {
        updater.start().await;
    });
    let bot_task = tokio::spawn(async move {
        bot.start().await;
    });
    let api_task = tokio::spawn(async move {
        if let Err(err) = start_api(api_state).await {
            eprintln!("API service failed: {}", err)
        };
    });

    // Graceful shutdown 
    match signal::ctrl_c().await {
        Ok(()) => {},
        Err(err) => {
            eprintln!("Unable to listen for shutdown signal: {}", err);
            updater_task.abort();
            bot_task.abort();
            api_task.abort();
        },
    }

    updater_task.abort();
    bot_task.abort();
    api_task.abort();
}
