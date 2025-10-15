use teloxide::{
    prelude::*,
    types::Message,
};
use crate::handlers::telegram_handler::handle_command;

pub struct BotService;

impl BotService {
    pub async fn run(bot_token: &str) {
        let bot = Bot::new(bot_token);

        teloxide::repl(bot, |bot, msg: Message| async move {
            handle_command(bot, msg).await;
            respond(())
        })
        .await;
    }
}