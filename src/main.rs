use anyhow::{Context, Result};
use rusty_money::{iso, Money};
use serde::Deserialize;
use teloxide::prelude::*;
use teloxide::types::{ChatId, Recipient};

#[derive(Deserialize, Debug)]
struct ApiResponse {
    symbol: String,
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

#[derive(Debug)]
struct Coin {
    symbol: String,
    price: f64,
}

/// Fetches a coin price from CoinPaprika API
async fn fetch_coin_info(client: &reqwest::Client, coin_id: &str) -> Result<Coin> {
    let url = format!("https://api.coinpaprika.com/v1/tickers/{}", coin_id);

    let response = client
        .get(&url)
        .send()
        .await
        .context("Failed to fetch coin price from CoinPaprika API")?;

    let data: ApiResponse = response
        .json()
        .await
        .context("Failed to parse CoinPaprika API response")?;

    Ok(Coin {
        symbol: data.symbol,
        price: data.quotes.usd.price,
    })
}

/// Parses channel ID from string (supports both @username and numeric formats)
fn parse_recipient(channel_str: &str) -> Result<Recipient> {
    if channel_str.starts_with('@') {
        Ok(Recipient::ChannelUsername(channel_str.to_string()))
    } else if let Ok(id) = channel_str.parse::<i64>() {
        Ok(Recipient::Id(ChatId(id)))
    } else {
        anyhow::bail!(
            "Invalid channel ID format: '{}'. Use numeric ID (e.g., -1001234567890) or @username",
            channel_str
        )
    }
}

async fn send_message_to_telegram(bot_token: &str, channel_id: &str, message: &str) -> Result<()> {
    let bot = Bot::new(bot_token);
    let recipient = parse_recipient(channel_id)?;

    bot.send_message(recipient, message)
        .parse_mode(teloxide::types::ParseMode::Html)
        .await
        .context("Failed to send message to Telegram channel")?;

    Ok(())
}

fn construct_message(coin: &Coin) -> String {
    let amount = Money::from_minor((coin.price * 100.0).round() as i64, iso::USD);
    format!("💰 {} Price: {}\n", coin.symbol, amount)
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let bot_token =
        std::env::var("TELOXIDE_TOKEN").context("TELOXIDE_TOKEN environment variable not set")?;
    let channel_id = std::env::var("TELEGRAM_CHANNEL_ID")
        .context("TELEGRAM_CHANNEL_ID environment variable not set")?;

    println!("📡 Fetching coin information...");

    let client = reqwest::Client::new();
    let coins = ["btc-bitcoin", "sol-solana"];
    let mut message = String::new();

    for coin_id in &coins {
        let coin = fetch_coin_info(&client, coin_id)
            .await
            .with_context(|| format!("Failed to fetch price for {}", coin_id))?;
        message.push_str(&construct_message(&coin));
    }

    println!("{}", message);
    println!("📤 Sending message to Telegram channel: {}", channel_id);

    send_message_to_telegram(&bot_token, &channel_id, &message).await?;

    println!("✅ Message sent successfully!");

    Ok(())
}
