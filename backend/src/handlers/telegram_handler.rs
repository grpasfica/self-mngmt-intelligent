use teloxide::{
    prelude::*,
    types::Message,
    utils::command::BotCommands,
};
use crate::domain::command::Command;
use crate::application::usecase::BotCommand;

pub async fn handle_command(bot: Bot, msg: Message) {
    if let Some(text) = msg.text() {
        // Parsing menjadi command
        if let Ok(cmd) = Command::parse(text, "self-mgmt-agent-bot") {
            if let Err(e) = BotCommand::handle_command(bot,  msg, cmd).await {
                eprintln!("Error handling command: {:?}", e);
            }
        }
    }
}