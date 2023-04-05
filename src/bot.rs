use teloxide::{prelude::*, utils::command::BotCommands};

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "Add new video/subscription")]
    Add,
    #[command(description = "Remove video/subscription")]
    Remove,
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
        Command::Add => bot.send_message(msg.chat.id, "Add").await?,
        Command::Remove => bot.send_message(msg.chat.id, "Remove").await?,
    };

    Ok(())
}
