use std::sync::Mutex;
use std::sync::Arc;
use anyhow::{Context, Result};
use teloxide::prelude::*;
use teloxide::types::MediaKind::{Text,Document};
use teloxide::types::MessageKind::{Common,ForumTopicCreated};
use clap::Parser;
use qbit_rs::Qbit;
use qbit_rs::model::Credential;

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


#[tokio::main]
async fn main() -> Result<(), Error> {

    let args = Args::parse();
    let config = load_config(&args).await.context("Failed to load config")?;

    let bot = Bot::new(config.token);

    let _qbit = Qbit::new(config.host.as_str(), Credential::new(config.username, config.password));

    teloxide::repl(bot, |_bot: Bot, msg: Message| async {
        let _qbit = _qbit.clone();
        //let _qbit = _qbit;
        println!("{:#?}",msg);
        if let Some(ref user) = msg.from {
            if user.id != UserId(config.user_id.try_into().unwrap()) {
                return Ok(())
            }
        } else {
            return Ok(())
        }

        if let Some(r) = msg.reply_to_message() {
            if let ForumTopicCreated(t) = &r.kind {
                let _category = &t.forum_topic_created.name;
                //_qbit.get_categories();
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
