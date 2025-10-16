#[derive(Clone)]
pub struct TelegramService;

impl TelegramService {
    pub fn new() -> Self {
        TelegramService
    }

    pub async fn process_message(&self, message: crate::domain::command::Message) {
        // Contoh: hanya print message
        println!("Processing message: {:?}", message);
    }
}
