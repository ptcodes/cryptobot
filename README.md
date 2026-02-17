# Crypto Telegram Price Bot 🪙

A Rust-based Telegram bot that automatically posts cryptocurrency price updates to a Telegram channel every hour using GitHub Actions. Currently tracks Bitcoin and Solana, with easy extensibility for additional cryptocurrencies.

## Features

- 🔄 **Automated hourly updates** via GitHub Actions
- 💰 **Multi-coin support** - Bitcoin, Solana, and easily extensible to other coins
- 📊 **Real-time prices** from CoinPaprika API
- 📱 **Clean formatted messages** with emoji and HTML formatting
- 🚀 **Simple deployment** - no server required
- 🔒 **Secure** - credentials stored in GitHub Secrets
- ⚡ **Fast and efficient** - runs once per execution, exits cleanly

## How It Works

1. GitHub Actions triggers the bot every hour via cron schedule
2. Bot fetches current prices for all configured cryptocurrencies from CoinPaprika API
3. Formats a nice message with prices for each coin
4. Sends the message to your configured Telegram channel
5. Exits (no continuous running process needed)

## Setup Instructions

### 1. Create a Telegram Bot

1. Open Telegram and search for [@BotFather](https://t.me/BotFather)
2. Send `/newbot` command
3. Follow the prompts to choose a name and username
4. Copy the bot token (you'll need this later)

### 2. Create a Telegram Channel

1. Create a new channel in Telegram
2. Add your bot as an administrator to the channel
3. Get your channel ID:
   - For public channels: use `@channelname` format
   - For private channels: use a bot like [@getidsbot](https://t.me/getidsbot) to get the numeric ID (e.g., `-1001234567890`)

### 3. Fork/Clone This Repository

```bash
git clone <your-repo-url>
cd cryptobot
```

### 4. Configure GitHub Secrets

1. Go to your GitHub repository
2. Navigate to **Settings** → **Secrets and variables** → **Actions**
3. Add the following secrets:
   - `TELOXIDE_TOKEN`: Your bot token from BotFather
   - `TELEGRAM_CHANNEL_ID`: Your channel ID (e.g., `@mychannel` or `-1001234567890`)

### 5. Enable GitHub Actions

1. Go to the **Actions** tab in your repository
2. Enable workflows if prompted
3. The bot will now run automatically every hour

### 6. Test Locally (Optional)

1. Create a `.env` file in the project root:

```bash
TELOXIDE_TOKEN=your_bot_token_here
TELEGRAM_CHANNEL_ID=@your_channel_here
```

2. Build and run:

```bash
cargo build --release
cargo run --release
```

3. Check your Telegram channel for the message

## Project Structure

```
cryptobot/
├── Cargo.toml                 # Project dependencies
├── .env                       # Local environment variables (gitignored)
├── .gitignore                 # Git ignore rules
├── src/
│   └── main.rs               # Main bot logic (coin configuration on line 105)
├── .github/
│   └── workflows/
│       └── hourly-update.yml  # GitHub Actions workflow
└── README.md                  # This file
```

## Configuration

### Environment Variables

- **TELOXIDE_TOKEN** (required): Your Telegram bot token
- **TELEGRAM_CHANNEL_ID** (required): Target channel ID

### Modify Update Frequency

Edit `.github/workflows/hourly-update.yml`:

```yaml
on:
  schedule:
    # Current: Every hour at minute 0
    - cron: '0 * * * *'

    # Examples:
    # Every 30 minutes: '*/30 * * * *'
    # Every 6 hours: '0 */6 * * *'
    # Daily at noon: '0 12 * * *'
```

## Troubleshooting

### Bot Not Sending Messages

1. **Check GitHub Actions logs**:
   - Go to **Actions** tab → Select latest workflow run
   - Review the logs for error messages

2. **Verify bot permissions**:
   - Ensure bot is an administrator in the channel
   - Check that bot can post messages

3. **Validate credentials**:
   - Double-check `TELOXIDE_TOKEN` in GitHub Secrets
   - Verify `TELEGRAM_CHANNEL_ID` format

4. **Test locally**:
   - Run the bot locally with `.env` file to isolate issues

### Common Errors

- **"TELOXIDE_TOKEN not set"**: Add the token to GitHub Secrets
- **"Failed to send message"**: Check bot permissions in channel
- **"Invalid channel ID"**: Use correct format (`@username` or numeric ID)
- **API rate limiting**: CoinPaprika free tier has rate limits; don't run too frequently

## Dependencies

- **teloxide** (0.14): Telegram bot framework
- **reqwest** (0.11): HTTP client for API calls
- **tokio** (1.x): Async runtime
- **serde/serde_json**: JSON serialization
- **anyhow**: Error handling
- **dotenv**: Environment variable loading
- **rusty-money** (0.5): Currency formatting

## API Information

### CoinPaprika API

- **Endpoint**: `https://api.coinpaprika.com/v1/tickers/{coin-id}`
- **Rate Limit**: 10-30 calls/minute (free tier)
- **No authentication required**
- **Documentation**: https://coinpaprika.com/api

### Currently Tracked Coins

- **Bitcoin (BTC)** - ID: `btc-bitcoin`
- **Solana (SOL)** - ID: `sol-solana`

### Adding More Coins

To add support for additional cryptocurrencies:

1. Find the coin ID from [CoinPaprika](https://coinpaprika.com) (format: `{symbol}-{name}`)
2. Open `src/main.rs`
3. Add the coin ID to the `coins` array on line 105:

```rust
let coins = ["btc-bitcoin", "sol-solana", "eth-ethereum"]; // Add your coin here
```

4. Commit and push your changes - the bot will automatically track the new coin!

## Future Enhancements

Potential improvements:

- [x] Support multiple cryptocurrencies ✅
- [ ] Price change percentage (24h)
- [ ] Price alerts for thresholds
- [ ] Historical price charts
- [ ] Price trend indicators
- [ ] Store price history

## License

MIT License - Feel free to use and modify!

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Disclaimer

This bot is for informational purposes only. Cryptocurrency prices are volatile. Do not make financial decisions based solely on this bot's data.
