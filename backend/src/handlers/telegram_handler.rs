use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};
use std::sync::Arc;
use serde::Deserialize;
use crate::{
    domain::command::{Message, User},
    services::bot_services::TelegramService
};
use tracing::info;

#[derive(Deserialize)]
pub struct TelegramUpdate{
    pub message: Option<TelegramMessage>
}

#[derive(Deserialize)]
pub struct TelegramMessage {
    pub message_id: i64,
    pub from: Option<TelegramUser>,
    pub text: Option<String>
}

#[derive(Deserialize)]
pub struct TelegramUser {
    pub id: i64,
    pub first_name: String,
    pub username: Option<String>
}

// Handler function for Telegram webhook
pub async fn telegram_webhook_handler(
    State(bot_services): State<Arc<TelegramService>>,
    Json(payload): Json<TelegramUpdate>
) -> impl IntoResponse {
    if let Some(msg) = payload.message {
        let user = msg.from.map(|u| User {
            id: u.id,
            name: u.first_name,
            username: u.username
        });

        let message = Message {
            id: msg.message_id,
            user,
            text: msg.text.unwrap_or_default()
        };

        info!("Received message: {:?}", message);

        bot_services.process_message(message).await;

        StatusCode::OK
    } else {
        StatusCode::BAD_REQUEST
    }
}