use std::sync::OnceLock;
use anyhow::Result;
use teloxide::prelude::*;
use teloxide::types::MediaKind::{Text,Document};
use teloxide::types::MessageKind::{Common,ForumTopicCreated};
use clap::Parser;
use qbit_rs::Qbit;
use qbit_rs::model::Credential;
use config::Config;

pub type Error = Box<dyn std::error::Error + Send + Sync>;

use crate::config::load_config;

mod config;

#[derive(Clone, Parser)]
#[clap(author, version, about)]
pub struct Args {
    /// Path to the config file. If not specified, it will default to /etc/qbitbot/config.toml
    #[clap(short, long)]
    pub config: Option<String>,
}

fn config() -> &'static Config<'static> {
    static CONFIG: OnceLock<Config> = OnceLock::new();
    CONFIG.get_or_init(|| {
        let args = Args::parse();
        let config = load_config(args).expect("Failed to load config");
        config
    })
}

fn qbit() -> &'static Qbit {
    static QBIT: OnceLock<Qbit> = OnceLock::new();
    QBIT.get_or_init(|| {
        Qbit::new(config().host, Credential::new(config().username, config().password))
    })
} 

#[tokio::main]
async fn main() -> Result<(), Error> {
    let config = config();
    let bot = Bot::new(config.token);

    teloxide::repl(bot, |_bot: Bot, msg: Message| async {
        println!("{:#?}",msg);
        if let Some(ref user) = msg.from {
            if user.id != UserId(config.user_id) {
                return Ok(())
            }
        } else {
            return Ok(())
        }

        if let Some(r) = msg.reply_to_message() {
            if let ForumTopicCreated(t) = &r.kind {
                println!("Hi");
                let _category = &t.forum_topic_created.name;
                qbit().get_categories();
            }
        }

        if let Common(m) = msg.kind {
            match m.media_kind {
                Text(t) => {
                    let _links = parse_magnets(t.text);
                },
                Document(_d) => {
                    println!("The hell do I do with this?");
                }
                _ => println!("I dunno inner"),
            }
        }
        Ok(())
    })
    .await;
    Ok(())
}

/// Replies to the user's text messages
async fn parse_magnets(message_text: String) -> Vec<String> {
    message_text.lines()
        .filter(|line| line.starts_with("magnet:"))
        .map(String::from)
        .collect()
}
