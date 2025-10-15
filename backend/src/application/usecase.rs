use crate::domain::command::Command;
use teloxide::{
    prelude::*,
    utils::command::BotCommands,
};

pub struct BotCommand;

impl BotCommand {
    pub async fn handle_command(bot: Bot, msg: Message, cmd: Command) -> anyhow::Result<()> {
        let chat_id = msg.chat.id;
        match cmd {
            Command::Start => {
                bot.send_message(
                    chat_id, 
                    "Halo! 👋. Saya adalah Self Management Intelligent Agent Anda. Bagaimana saya bisa membantu Anda hari ini?"
                ).await?;
            }
            Command::Hallo => {
                bot.send_message(
                    chat_id, 
                    "Hallo 👋, Adakah yang bisa saya bantu?"
                ).await?;
            }
            Command::Help => {
                bot.send_message(
                    chat_id, 
                    Command::descriptions().to_string()
                ).await?;
            }
            Command::Stop => {
                bot.send_message(
                    chat_id,
                    "Baiklah, sampai jumpa lagi! 👋"
                ).await?;
                // Log pesan saat bot berhenti
                println!("🛑 Bot stopped ...");
                std::process::exit(0);
            }
        }
        Ok(())
    }
}