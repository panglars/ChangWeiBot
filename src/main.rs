use rusqlite::Connection;

use teloxide::{
    prelude::*,
    utils::command::{self, BotCommands},
};

mod sql;
mod stats_api;

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
    #[command(description = "Display this text")]
    Help,
    #[command(description = "Bind ea username")]
    Bind(String),
    #[command(description = "Unbind ea username")]
    Unbind(String),
    #[command(description = "Show weapons stats")]
    Weapons(String),
    #[command(description = "Show vehicles stats")]
    Vehicles(String),
    #[command(description = "Show player status")]
    Status(String),
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    let conn = Connection::open("telegram_users.db").expect("error open db");
    //    println!("msg: {:#?}", &msg);

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
        Command::Status(username) => {
            if username.is_empty() {
                let ea_id = sql::query_user(&conn, &msg.from().unwrap().id.to_string())
                    .expect("failed to check");
                stats_api::get_stats(&ea_id);
                bot.send_message(msg.chat.id, format!("Checking {ea_id}"))
                    .await?
            } else {
                bot.send_message(msg.chat.id, format!("Checking {username}."))
                    .await?
            }
        }

        Command::Bind(username) => {
            if msg.chat.id.is_user() {
                sql::insert_user(&conn, &msg.from().unwrap().id.to_string(), &username)
                    .expect("failed insert");
                bot.send_message(msg.chat.id, format!("Bind with {username}."))
                    .await?
            } else {
                bot.send_message(msg.chat.id, format!("Please PM to bind user."))
                    .await?
            }
        }
        Command::Unbind(username) => {
            if msg.chat.id.is_user() {
                sql::delete_user(&conn, &msg.from().unwrap().id.to_string())
                    .expect("Failed to delete");
                bot.send_message(msg.chat.id, format!("Unbind with {username}."))
                    .await?
            } else {
                bot.send_message(msg.chat.id, format!("Please PM to unbind user."))
                    .await?
            }
        }
    };

    Ok(())
}
