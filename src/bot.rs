mod subscription_parser;

use teloxide::{prelude::*, utils::command::BotCommands};
use crate::store::Store;

use crate::models::subscription::Subscription;
use self::subscription_parser::subscription_parser;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "Add new video/subscription", parse_with = subscription_parser)]
    Add(String, Subscription),
    #[command(description = "Remove subscription", parse_with = subscription_parser)]
    Remove(String, Subscription),
}

pub struct BotSerivce {
    bot: Bot,
    store: Store
}

impl BotSerivce {
    pub fn init(token: String, store: Store) -> Self {
        Self { 
            bot: Bot::new(token),
            store,
        }
    }

    pub async fn start(&self) {
        let bot = self.bot.clone();
        let store = self.store.clone();
        
        Command::repl(bot, move |bot, msg, cmd| {
            answer(bot, msg, cmd, store.clone())
        }).await;
    } 
}

async fn answer(bot: Bot, msg: Message, cmd: Command, store: Store) -> ResponseResult<()> {
    match cmd {
        Command::Add(_id, sub) => {
            // TODO: a.FeedService.Add(msg.Arguments) instead of Store
            match store.create_subscription(sub.clone()).await {
                Ok(_) => bot.send_message(msg.chat.id, "Subscribed").await?,
                Err(_) => bot.send_message(msg.chat.id, "Failed to add subscription. See logs for more info").await?
            }
        },
        Command::Remove(_id, sub) => {
            match store.delete_subscription(sub.clone()).await {
                Ok(_) => bot.send_message(msg.chat.id, "Unsubscribed").await?,
                Err(_) => bot.send_message(msg.chat.id, "Failed to remove subscription. See logs for more info").await?
            }
        }
    };

    Ok(())
}
