#[macro_use]
extern crate log;

mod models;
mod store;
mod file_manager;
mod feed_service;
mod bot;
mod updater;

use std::env;
use dotenv::dotenv;
use tokio::signal;

use store::Store;
use file_manager::FileManager;
use feed_service::FeedService;
use bot::BotSerivce;
use updater::Updater;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    // FIXME: Logs are not visible in terminal
    info!("Starting service");

    // Check for required parameters
    let update_interval = env::var("UPDATE_INTERVAL").unwrap().parse::<u64>().expect("Update Interval not set");
    let token = env::var("BOT_TOKEN").expect("Bot token not set");

    // Services
    let store = Store::init().await.expect("Database can not be inited");
    let file_manager = FileManager{};
    let feed_service = FeedService::init(store, file_manager);

    // Handlers
    let updater = Updater::init(update_interval * 60, feed_service.clone());
    let bot = BotSerivce::init(token, feed_service);

    // Start handlers
    let updater_task = tokio::spawn(async move {
        updater.start().await;
    });
    let bot_task = tokio::spawn(async move {
        bot.start().await;
    });

    // Graceful shutdown 
    match signal::ctrl_c().await {
        Ok(()) => {},
        Err(err) => {
            eprintln!("Unable to listen for shutdown signal: {}", err);
            updater_task.abort();
            bot_task.abort();
        },
    }

    updater_task.abort();
    bot_task.abort();
}
