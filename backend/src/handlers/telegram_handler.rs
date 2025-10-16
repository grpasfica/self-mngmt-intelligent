use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
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

// Struktur untuk response body
#[derive(Serialize)]
pub struct ApiResponse {
    pub success: bool,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>
}

impl ApiResponse {
    pub fn success(message: impl Into<String>) -> Self {
        Self {
            success: true,
            message: message.into(),
            data: None
        }
    }

    pub fn success_with_data(message: impl Into<String>, data: serde_json::Value) -> Self {
        Self {
            success: true,
            message: message.into(),
            data: Some(data)
        }
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self {
            success: false,
            message: message.into(),
            data: None
        }
    }
}

// Handler function dengan response body
pub async fn telegram_webhook_handler(
    State(bot_services): State<Arc<TelegramService>>,
    Json(payload): Json<TelegramUpdate>,
) -> Response {
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

        // Response sukses dengan body
        (
            StatusCode::OK,
            Json(ApiResponse::success("Message processed successfully"))
        ).into_response()
    } else {
        // Response gagal dengan body
        (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::error("Invalid request: message field is missing"))
        ).into_response()
    }
}