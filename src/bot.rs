mod command_parser;

use teloxide::{prelude::*, utils::command::BotCommands};
use crate::feed_service::FeedService;

use crate::models::youtube_item::YouTubeItem;
use self::command_parser::parse_youtube_item;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "Add new video/subscription", parse_with = parse_youtube_item)]
    Add(String, YouTubeItem),
    #[command(description = "Remove subscription", parse_with = parse_youtube_item)]
    Remove(String, YouTubeItem),
}

pub struct BotSerivce {
    bot: Bot,
    feed_service: FeedService
}

impl BotSerivce {
    pub fn init(token: String, feed_service: FeedService) -> Self {
        Self { 
            bot: Bot::new(token),
            feed_service,
        }
    }

    pub async fn start(&self) {
        let bot = self.bot.clone();
        let feed_service = self.feed_service.clone();
        
        Command::repl(bot, move |bot, msg, cmd| {
            answer(bot, msg, cmd, feed_service.clone())
        }).await;
    } 
}

async fn answer(bot: Bot, msg: Message, cmd: Command, feed_service: FeedService) -> ResponseResult<()> {
    match cmd {
        Command::Add(_id, item) => {
            match feed_service.add(item).await {
                Ok(_) => bot.send_message(msg.chat.id, "Subscribed").await?,
                Err(err) => {
                    println!("{}", err);
                    bot.send_message(msg.chat.id, "Failed to add subscription. See logs for more info").await?
                }
            }
        },
        Command::Remove(_id, item) => {
            match feed_service.delete(item).await {
                Ok(_) => bot.send_message(msg.chat.id, "Unsubscribed").await?,
                Err(_) => bot.send_message(msg.chat.id, "Failed to remove subscription. See logs for more info").await?
            }
        }
    };

    Ok(())
}
