#[macro_use]
extern crate log;

use std::env;
use dotenv::dotenv;

mod models;
mod bot;
mod store;

use bot::BotSerivce;
use store::Store;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    // FIXME: Logs are not visible in terminal
    info!("Starting bot service");

    let store = Store::init().await.expect("Database can not be inited");

    let token = env::var("BOT_TOKEN").expect("Bot token not set");
    let bot = BotSerivce::init(token, store);

    // TODO: Add grasefull shutdown
    // TODO: Run in separate thread
    bot.start().await;
}
