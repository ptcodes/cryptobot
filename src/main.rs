use anyhow::{Context, Result};
use rusty_money::{iso, Money};
use serde::Deserialize;
use teloxide::prelude::*;
use teloxide::types::{ChatId, Recipient};

#[derive(Deserialize, Debug)]
struct ApiResponse {
    quotes: Quote,
}

#[derive(Deserialize, Debug)]
struct Quote {
    #[serde(rename = "USD")]
    usd: UsdQuote,
}

#[derive(Deserialize, Debug)]
struct UsdQuote {
    price: f64,
}

/// Fetches the current Bitcoin price from CoinPaprika API
async fn fetch_bitcoin_price() -> Result<f64> {
    let url = "https://api.coinpaprika.com/v1/tickers/btc-bitcoin";

    let response = reqwest::get(url)
        .await
        .context("Failed to fetch Bitcoin price from CoinPaprika API")?;

    let data: ApiResponse = response
        .json()
        .await
        .context("Failed to parse CoinPaprika API response")?;

    Ok(data.quotes.usd.price)
}

/// Parses channel ID from string (supports both @username and numeric formats)
fn parse_recipient(channel_str: &str) -> Result<Recipient> {
    if channel_str.starts_with('@') {
        // Username format (e.g., @mychannel)
        Ok(Recipient::ChannelUsername(channel_str.to_string()))
    } else if let Ok(id) = channel_str.parse::<i64>() {
        // Numeric ID format (e.g., -1001234567890)
        Ok(Recipient::Id(ChatId(id)))
    } else {
        anyhow::bail!(
            "Invalid channel ID format: '{}'. Use numeric ID (e.g., -1001234567890) or @username",
            channel_str
        )
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables from .env file (for local testing)
    dotenv::dotenv().ok();

    println!("🤖 Starting Bitcoin Price Bot...");

    // Get environment variables
    let bot_token =
        std::env::var("TELOXIDE_TOKEN").context("TELOXIDE_TOKEN environment variable not set")?;

    let channel_id_str = std::env::var("TELEGRAM_CHANNEL_ID")
        .context("TELEGRAM_CHANNEL_ID environment variable not set")?;

    println!("📡 Fetching Bitcoin price...");

    // Fetch Bitcoin price
    let price = fetch_bitcoin_price()
        .await
        .context("Failed to fetch Bitcoin price")?;

    // Make the price look nicer
    let amount = Money::from_major(price as i64, iso::USD);

    let message = format!("💰 Current Bitcoin Price: {}", amount);
    println!("{}", message);

    println!("📤 Sending message to Telegram channel: {}", channel_id_str);

    // Initialize bot
    let bot = Bot::new(bot_token);

    // Parse channel recipient
    let recipient = parse_recipient(&channel_id_str)?;

    // Send message to channel
    bot.send_message(recipient, &message)
        .parse_mode(teloxide::types::ParseMode::Html)
        .await
        .context("Failed to send message to Telegram channel")?;

    println!("✅ Message sent successfully!");
    println!("🎉 Bot execution completed!");

    Ok(())
}
