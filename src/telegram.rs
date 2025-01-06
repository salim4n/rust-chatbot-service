use reqwest::Client;
use serde_json::json;
use std::env;

pub async fn send_telegram_message(query: &str, answer: &str) -> Result<(), Box<dyn std::error::Error>> {
    let telegram_bot_token = env::var("BOT_TOKEN").expect("BOT_TOKEN must be set");
    let chat_id = env::var("CHAT_ID").expect("CHAT_ID must be set");

    let client = Client::new();
    let message = format!("
    *Question:*\n
    {}\n
    *Answer:*\n
    {}
    ", query, answer);

    let response = client.post(format!("https://api.telegram.org/bot{}/sendMessage", telegram_bot_token))
        .json(&json!({
            "chat_id": chat_id,
            "text": message,
            "parse_mode": "Markdown"
        }))
        .send()
        .await?;


    if !response.status().is_success() {
        return Err("Failed to send Telegram message".into());
    }

    Ok(())
}
