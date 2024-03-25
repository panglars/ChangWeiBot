use rusqlite::Connection;

use teloxide::{
    prelude::*,
    types::MessageCommon,
    utils::command::{self, BotCommands},
};

mod sql;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting command bot...");

    let bot = Bot::from_env();

    Command::repl(bot, answer).await;
}

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "handle a username.")]
    Weapons(String),
    #[command(description = "handle a username and an age.")]
    Vehicles(String),
    #[command(description = "bind username")]
    Bind(String),
    #[command(description = "bind username")]
    Status(String),
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    let conn = Connection::open("telegram_users.db").expect("error open db");
    println!("msg: {:#?}", msg);
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?
        }
        // TODO archive
        Command::Weapons(username) => {
            bot.send_message(msg.chat.id, format!("Your username is @{username}."))
                .await?
        }
        Command::Vehicles(username) => {
            bot.send_message(msg.chat.id, format!("Your username is @{username}."))
                .await?
        }
        Command::Bind(username) => {
            if msg.chat.id.is_user() {
                sql::insert_user(&conn, &msg.chat.id.to_string(), &username)
                    .expect("failed insert");
                bot.send_message(msg.chat.id, format!("Bind with {username}."))
                    .await?
            } else {
                bot.send_message(msg.chat.id, format!("Please PM to bind user."))
                    .await?
            }
        }
        Command::Status(username) => {
            if msg.chat.id.is_user() {
                if username.is_empty() {
                    let ea_id =
                        sql::query_user(&conn, &msg.chat.id.to_string()).expect("failed to check");
                    bot.send_message(msg.chat.id, format!("Checking {ea_id}"))
                        .await?
                } else {
                    bot.send_message(msg.chat.id, format!("Checking {username}."))
                        .await?
                }
            } else {
                let sender;
                if let Some(MessageKind::Common(common)) = &msg.kind {
                    if let Some(user) = common.from {
                        sender = user.id;
                        println!("User ID: {}", user.id);
                    }
                }
                bot.send_message(msg.chat.id, format!("Checking {sender}."))
                    .await?
            }
        }
    };

    Ok(())
}
