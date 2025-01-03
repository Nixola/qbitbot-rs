use anyhow::{bail, Context, Result};
use teloxide::{prelude::*, types::User};
use teloxide::types::MediaKind::{Text,Document};
use teloxide::types::MessageKind::Common;
use clap::{Parser, Command};
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

    let qbit = Qbit::new(config.host.as_str(), Credential::new(config.username, config.password));
    println!("{:?}", config.map);

    teloxide::repl(bot, move |bot: Bot, msg: Message| async move {
        println!("{:#?}",msg);
        if let Some(ref user) = msg.from {
            if user.id != UserId(config.user_id.try_into().unwrap()) {
                return Ok(())
            }
        } else {
            return Ok(())
        }

        match msg.kind {
            Common(m) => match m.media_kind {
                Text(t) => {
                    parse_magnets(bot, msg.from.unwrap(), t.text);
                },
                Document(d) => {
                    println!("The hell do I do with this?");
                }
                _ => println!("I dunno inner"),
            },
            _ => println!("I dunno outer"),
        }
        Ok(())
    })
    .await;
    Ok(())
}

/// Replies to the user's text messages
async fn parse_magnets(bot: Bot, user: User, message_text: String) -> Result<(), Error> {
    /*
       The id of a chat with a user is the same as his telegram_id
       from the bot's perspective.

       Injected dependencies:
       - Bot is provided by the Dispatcher::dispatch
       - User is provided by the (1)
       - String is provided by the (2)
    */
    let links = message_text.lines()
        .filter(|line| line.starts_with("magnet:"));

    let count = links.count();

    if count > 0 {
        println!("{}",count);
    }
    
    //bot.send_message(user.id, format!("Hi! You sent: {message_text}")).send().await;
    Ok(())
}
