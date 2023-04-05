use teloxide::{prelude::*, utils::command::BotCommands};
use crate::commands::{Subscription, subscription_parser};

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "Add new video/subscription", parse_with = subscription_parser)]
    Add(String, Subscription),
    #[command(description = "Remove video/subscription", parse_with = subscription_parser)]
    Remove(String, Subscription),
}

pub struct BotSerivce {
    bot: Bot
}

impl BotSerivce {
    pub fn init(token: String) -> Self {
        Self { 
            bot: Bot::new(token),
        }
    }

    pub async fn start(&self) {
        let bot = self.bot.clone();
        Command::repl(bot, answer).await;
    }
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Add(_id, sub) => {
            bot.send_message(
                msg.chat.id, 
                sub.to_string()
            ).await?
        },
        Command::Remove(_id, sub) => {
            bot.send_message(
                msg.chat.id, 
                sub.to_string()
            ).await?
        },
    };

    Ok(())
}
