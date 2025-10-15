use teloxide_macros::BotCommands;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "✨ Perintah yang tersedia saat ini: ✨")]
pub enum Command{
    #[command(description = "Agent started\n")]
    Start,
    #[command(description = "Agent says Hallo\n")]
    Hallo,
    #[command(description = "Agent helper\n")]
    Help,
    #[command(description = "Stop Agent\n")]
    Stop,
}