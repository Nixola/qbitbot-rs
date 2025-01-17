qbitbot-rs
======

A Telegram bot to add torrent files and links to a qBittorrent instance. Why? Hell if I know, I thought it'd be handy to use and fun to make.

Installation
------------

### Cargo

```bash
$ cargo install --git=https://github.com/Nixola/qbitbot-rs
```

Configuration
-------------

I'm probably going to forget to keep this readme up to date. Check `config.rs` when in doubt.  
I should probably add comments there.

Put a `config.toml` in a config directory such as `~/.config/qbitbot/` or `/etc/qbitbot/` with the following contents.

```toml
# Telegram bot token, in quotes.
token = ""
# Telegram user ID. Messages sent by any other user are ignored.
user_id = 1
# qBittorrent webui/api username, in quotes.
username = ""
# qBittorrent webui/api password, in quotes.
password = ""
# qBittorrent webui/api address, including the port, in quotes.
host = "http://192.168.1.96:8000"
```

Alternatively, the following environment variables have the same effect and take precedence:
```shell
QBITBOT_TOKEN=""
QBITBOT_USER_ID=1
QBITBOT_USERNAME=""
QBITBOT_PASSWORD=""
QBITBOT_HOST=""
```

Usage
-----

Create or edit your bot in Botfather disabling group privacy mode, then add the bot to a group chat. Enable topics in said group chat, and create a topic (sending a message as well) for each category you want to be able to add torrents to. The bot supports both torrent files and magnet links, and other links probably work as well.

After that, get the bot running!
```bash
$ qbitbot
```
Make a systemd service for it, or leave it running in `screen` or `tmux` or whatever.
