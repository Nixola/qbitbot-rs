use crate::config::Config;
use anyhow::Result;
use clap::Parser;
use qbit_rs::model::{AddTorrentArg, Credential, TorrentFile, TorrentSource};
use qbit_rs::Qbit;
use std::sync::Arc;
use teloxide::net::Download;
use teloxide::prelude::*;
use teloxide::types::MediaKind::{Document, Text};
use teloxide::types::MessageKind::{Common, ForumTopicCreated};
use teloxide::types::ReplyParameters;
use url::Url;

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
    let config = Arc::new(load_config(args).expect("Failed to load config"));
    let qbit = Arc::new(Qbit::new(
        Url::parse(&config.host).expect("Invalid qBittorrent URL provided"),
        Credential::new(&config.username, &config.password),
    ));
    let bot = Bot::new(&config.token);

    let handler = Update::filter_message().endpoint(
        |bot: Bot, qbit: Arc<Qbit>, config: Arc<Config>, msg: Message| async move {

            // Ignore messages from any user other than the one in the configuration
            let Some(ref user) = msg.from else {
                return respond(());
            };
            if user.id != UserId(config.user_id) {
                return respond(());
            }

            // Evaluate the incoming message for magnet links (if text) or torrent files (if document)
            let source =
            // Honestly, I'm not entirely sure what I'm doing here, but I think I have to?
            if let Common(ref m) = msg.kind {
                match &m.media_kind {
                    // Parse magnet links
                    Text(t) => {
                        let links = parse_magnets(&t.text).await;
                        // Abort if no links are present
                        if links.is_empty() { return respond(()) };
                        TorrentSource::Urls {urls: links.into()}
                    },
                    // Apparently, there's only one document per message? Multiple attachments are sent as multiple messages as far as I can tell
                    Document(d) => {
                        // Reject non-torrent files, and ones without a mimetype
                        let Some(ref mime_type) = &d.document.mime_type else { return respond(()) };
                        if *mime_type != "application/x-bittorrent".parse::<mime::Mime>().unwrap() {
                            return respond(())
                        }

                        // Reject nameless files. Can this even happen?
                        let file_name = match &d.document.file_name {
                            Some(n) => n,
                            None => return respond(()),
                        };

                        // Download the file from Telegram
                        let file_path = bot.get_file(&d.document.file.id).await?;
                        let mut file_contents: Vec<u8> = Vec::new();
                        if let Err(e) = bot.download_file(&file_path.path, &mut file_contents).await {
                            // Warn the user, then abort
                            bot.send_message(msg.chat.id, format!("Couldn't download file, please retry\n```{:#?}```", e))
                                .reply_parameters(ReplyParameters::new(msg.id))
                                .await?;
                            return respond(())
                        }

                        // qbittorrent wants both filename and file content
                        let torrent_file = TorrentFile {filename: file_name.to_string(), data: file_contents};
                        TorrentSource::TorrentFiles {torrents: vec![torrent_file]}
                    },
                    // Ignore anything that's not a text or document message
                    _ => return respond(())
                }
            } else {
                return respond(());
            };

            // Get the category from the topic name, which can be retrieved from the message this message is replying to
            // Note: replying to a message prevents this from working, I'd need to go up the chain and I'm not bothering
            // Note: renaming the topic won't work, as the ForumTopicCreated still contains the original name! Recreate it!
            let category = if let Some(r) = msg.reply_to_message() {
                if let ForumTopicCreated(t) = &r.kind {
                    // Get the category list from qbittorrent, abort if nonexistent
                    let categories = qbit.get_categories().await.ok().unwrap();
                    match categories.get(&t.forum_topic_created.name) {
                        Some(category) => {
                            category.name.clone()
                        },
                        None => {
                            return respond(())
                        }
                    }
                // If the message is a reply, ignore it
                } else {
                    return respond(())
                }
            // If the message isn't in a topic, ignore it
            } else {
                return respond(())
            };

            let add_torrent_arg = AddTorrentArg::builder()
                .source(source)
                .category(category)
                .auto_torrent_management(true)
                .build();
            if let Err(e) = qbit.add_torrent(add_torrent_arg).await {
                bot.send_message(msg.chat.id, format!("Couldn't add torrent\n{:#?}", e))
                    .reply_parameters(ReplyParameters::new(msg.id))
                    .await?;
            }

            respond(())
        },
    );

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![qbit, config])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
    Ok(())
}

// Parse a string so as to extract lines that form valid URLs
async fn parse_magnets(message_text: &str) -> Vec<Url> {
    message_text
        .lines()
        .map(Url::parse)
        .filter(|url| url.clone().ok().is_some())
        .map(|url| url.unwrap())
        .collect()
}
