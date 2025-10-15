mod application;
mod domain;
mod handlers;
mod services;

use dotenvy::dotenv;
use std::env;
use services::bot_services::BotService;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let bot_token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN must be set");
    
    // Log pesan bot
    println!("🤖 Bot started ...");

    BotService::run(&bot_token).await;
}