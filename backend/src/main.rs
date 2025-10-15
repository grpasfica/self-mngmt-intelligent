use teloxide::{
    prelude::*, 
    types::Message,
    utils::command::BotCommands,
};
use teloxide_macros::BotCommands;
use dotenvy::dotenv;
use std::env;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "✨ Perintah yang tersedia saat ini: ✨")]
enum Command{
    #[command(description = "Agent started\n")]
    Start,
    #[command(description = "Agent says Hallo\n")]
    Hallo,
    #[command(description = "Agent helper\n")]
    Help,
    #[command(description = "Stop Agent\n")]
    Stop,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let bot_token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN tidak tersedia di .env");

    let bot = Bot::new(bot_token);

    // Log pesan bot
    println!("🤖 Bot started ...");

    teloxide::repl(bot, |bot: Bot, msg: Message| async move{
        if let Some(text) = msg.text() {
            // Parsing menjadi command
            if let Ok(cmd) = Command::parse(text, "self-mgmt-agent-bot") {
                match cmd {
                    Command::Start => {
                        bot.send_message(
                            msg.chat.id, 
                            "Halo! 👋. Saya adalah Self Management Intelligent Agent Anda. Bagaimana saya bisa membantu Anda hari ini?"
                        ).await?;
                    }
                    Command::Hallo => {
                        bot.send_message(
                            msg.chat.id, 
                            "Hallo 👋, Adakah yang bisa saya bantu?"
                        ).await?;
                    }
                    Command::Help => {
                        bot.send_message(
                            msg.chat.id, 
                            Command::descriptions().to_string()
                        ).await?;
                    }
                    Command::Stop => {
                        bot.send_message(
                            msg.chat.id,
                            "Baiklah, sampai jumpa lagi! 👋"
                        ).await?;
                        std::process::exit(0);
                    }
                }
            }
        }
        respond(())
    }).await;
}