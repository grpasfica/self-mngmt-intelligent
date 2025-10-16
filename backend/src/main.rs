use axum::{
    routing::post,
    Router
};
use std::{
    net::SocketAddr,
    sync::Arc
};
use tracing_subscriber;
use tokio::net::TcpListener;
use backend::services::bot_services::TelegramService;
use backend::handlers::telegram_handler::telegram_webhook_handler;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let bot_services = Arc::new(TelegramService::new());

    let app = Router::new()
        .route("/telegram/webhook", post(telegram_webhook_handler))
        .with_state(bot_services);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));

    let listener = TcpListener::bind(addr)
        .await
        .expect("❌ Failed to bind to address");

    println!("✅ Server running at {}", addr);

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}