use bot::BotSerivce;

#[macro_use]
extern crate log;

use std::env;
use dotenv::dotenv;

mod bot;
mod commands;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    // TODO: Logs are not visible in terminal
    info!("Starting bot service");

    let token = env::var("BOT_TOKEN").expect("Bot token not set");
    let bot = BotSerivce::init(token);
    bot.start().await;
}