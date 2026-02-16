# Quick Setup Guide

## Prerequisites
- Telegram account
- GitHub account
- Rust installed locally (for testing)

## Step-by-Step Setup

### 1. Create Telegram Bot (5 minutes)

1. Open Telegram and search for `@BotFather`
2. Send `/newbot` command
3. Choose a name (e.g., "Bitcoin Price Bot")
4. Choose a username (must end in 'bot', e.g., "my_bitcoin_price_bot")
5. **Copy the bot token** - you'll need it later!

### 2. Create Telegram Channel (2 minutes)

1. Create a new channel in Telegram
2. Make it public or private (your choice)
3. Add your bot as an administrator:
   - Go to channel settings
   - Administrators → Add Administrator
   - Search for your bot username
   - Grant "Post Messages" permission
4. Get your channel ID:
   - **Public channel**: Use `@channelname` (e.g., `@mybitcoinchannel`)
   - **Private channel**: Forward a message from the channel to `@getidsbot` to get the numeric ID (e.g., `-1001234567890`)

### 3. Test Locally (Optional but Recommended)

1. Edit the `.env` file in this directory:
```bash
TELOXIDE_TOKEN=123456789:ABCdefGHIjklMNOpqrsTUVwxyz
TELEGRAM_CHANNEL_ID=@mybitcoinchannel
```

2. Run the bot:
```bash
cargo run --release
```

3. Check your Telegram channel - you should see a Bitcoin price message!

### 4. Deploy to GitHub Actions

1. **Push code to GitHub**:
```bash
git init
git add .
git commit -m "Initial commit: Bitcoin Telegram bot"
git remote add origin https://github.com/yourusername/yourrepo.git
git push -u origin main
```

2. **Add GitHub Secrets**:
   - Go to your repository on GitHub
   - Click **Settings** → **Secrets and variables** → **Actions**
   - Click **New repository secret**
   - Add these two secrets:
     - Name: `TELOXIDE_TOKEN`, Value: (your bot token)
     - Name: `TELEGRAM_CHANNEL_ID`, Value: (your channel ID)

3. **Enable GitHub Actions**:
   - Go to the **Actions** tab
   - If prompted, click "I understand my workflows, go ahead and enable them"

4. **Test the workflow manually**:
   - Go to **Actions** tab
   - Click on "Hourly Bitcoin Price Update" workflow
   - Click **Run workflow** → **Run workflow**
   - Wait a minute and check your channel!

### 5. Verify Automatic Updates

The bot will now automatically post Bitcoin prices every hour at minute 0 (e.g., 10:00, 11:00, 12:00, etc.).

To verify:
- Wait until the next hour
- Check the **Actions** tab for workflow runs
- Check your Telegram channel for new messages

## Troubleshooting

### "TELOXIDE_TOKEN environment variable not set"
- **Local**: Make sure `.env` file exists and has the correct token
- **GitHub**: Verify the secret is added in repository settings

### "Failed to send message to Telegram channel"
- Make sure bot is an administrator in the channel
- Verify channel ID format (use `@channelname` or numeric ID)
- Check that bot has "Post Messages" permission

### Workflow not running
- Check that GitHub Actions is enabled
- Verify the workflow file exists at `.github/workflows/hourly-update.yml`
- Look at the Actions tab for any error messages

### Want to change update frequency?
Edit `.github/workflows/hourly-update.yml` and change the cron schedule:
- Every 30 minutes: `*/30 * * * *`
- Every 6 hours: `0 */6 * * *`
- Twice daily (noon and midnight): `0 0,12 * * *`

## Success Checklist

- [ ] Created bot with @BotFather
- [ ] Created Telegram channel
- [ ] Added bot as channel administrator
- [ ] Tested locally (optional)
- [ ] Pushed code to GitHub
- [ ] Added GitHub secrets
- [ ] Manually triggered workflow successfully
- [ ] Verified automatic hourly updates

## Support

If you encounter issues:
1. Check the workflow logs in GitHub Actions tab
2. Verify all secrets are correctly set
3. Test locally first to isolate the problem
4. Review the README.md for more detailed information
