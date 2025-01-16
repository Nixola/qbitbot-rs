Hermes
======

A Telegram bot to add torrent files to a qBittorrent instance. Why? Hell if I know, I thought it'd be handy to use and fun to make.

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
# Chat ID. Currently ignored. I should probably delete it.
chat_id = 2
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
QBITBOT_CHAT_ID=2
QBITBOT_USERNAME=""
QBITBOT_PASSWORD=""
QBITBOT_HOST=""
```

Usage
-----

Just run the thing.
```bash
$ qbitbot
```
Make a systemd service for it, or leave it running in `screen` or `tmux` or whatever.