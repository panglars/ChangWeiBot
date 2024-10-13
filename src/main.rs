use changweibot::backend::{backend, req, ConsumerChan, ProducerChan, StateRequest, StateResponse};
use changweibot::utilities::{get_ea_id, make_vehicle_keyboard};
use log::info;
use teloxide::{prelude::*, utils::command::BotCommands};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    log::info!("Starting command bot...");
    let bot = Bot::from_env();
    let (tx, rx): (ProducerChan, ConsumerChan) = tokio::sync::mpsc::channel(16);
    let tx2 = tx.clone();
    let backend_handler = tokio::spawn(async { backend(rx).await });
    let start_time = chrono::Utc::now();
    // TODO: replace to dispatching model
    Command::repl(bot, move |bot: Bot, msg: Message, cmd: Command| {
        let tx = tx.clone();
        let start_time = start_time.clone();
        async move {
            if msg.date < start_time {
                log::warn!("Ignored out-of-date message: {}", msg.id.0);
                return ResponseResult::Ok(());
            }
            answer(tx, bot, msg, cmd).await
        }
    })
    .await;
    log::info!("Stopping backend...");
    req(tx2, StateRequest::Stop).await;
    backend_handler.await.unwrap();
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
    Unbind,
    #[command(description = "Show weapons stats")]
    Weapons(String),
    #[command(description = "Show vehicles stats")]
    Vehicles(String),
    #[command(description = "Show player status")]
    Status(String),
}

async fn answer(chan: ProducerChan, bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?
        }
        Command::Weapons(username) => {
            let ea_id = match get_ea_id(chan.clone(), &bot, &msg, username).await {
                Ok(id) => id,
                Err(_) => return Ok(()),
            };
            let json = match req(
                chan,
                StateRequest::GetWeapons {
                    ea_id: ea_id.clone(),
                },
            )
            .await
            {
                StateResponse::Weapons(s) => s,
                _ => {
                    bot.send_message(
                        msg.chat.id,
                        format!("Failed to fetch your EA stats, please wait a while and retry or recheck your username."),
                    )
                    .await?;
                    return Ok(());
                }
            };
            info!("Checking {ea_id} weapons stats.");
            bot.send_message(msg.chat.id, format!("Weapons of {ea_id}:\n{:#?}", json))
                .await?
        }

        Command::Vehicles(username) => {
            let keyboard = make_vehicle_keyboard();
            let ea_id = match get_ea_id(chan.clone(), &bot, &msg, username).await {
                Ok(id) => id,
                Err(_) => return Ok(()),
            };
            let json = match req(
                chan,
                StateRequest::GetVehicles {
                    ea_id: ea_id.clone(),
                },
            )
            .await
            {
                StateResponse::Vehicles(s) => s,
                _ => {
                    bot.send_message(
                        msg.chat.id,
                        "Failed to fetch your EA stats, please wait a while and retry.",
                    )
                    .await?;
                    return Ok(());
                }
            };

            info!("Checking {ea_id} vehicles stats.");
            bot.send_message(msg.chat.id, format!("Vehicles of {ea_id}:\n{:#?}", json))
                .reply_markup(keyboard)
                .await?
        }

        Command::Status(username) => {
            let ea_id = match get_ea_id(chan.clone(), &bot, &msg, username).await {
                Ok(id) => id,
                Err(_) => return Ok(()),
            };
            let json = match req(
                chan,
                StateRequest::GetStats {
                    ea_id: ea_id.clone(),
                },
            )
            .await
            {
                StateResponse::Stats(s) => s,
                _ => {
                    bot.send_message(
                        msg.chat.id,
                        "Failed to fetch your EA stats, please wait a while and retry.",
                    )
                    .await?;
                    return Ok(());
                }
            };

            info!("Checking {ea_id} status.");

            bot.send_message(msg.chat.id, format!("Status of {ea_id}:\n{:#?}", json))
                .await?
        }

        Command::Bind(username) => {
            if !msg.chat.id.is_user() {
                bot.send_message(msg.chat.id, format!("Please PM to bind user."))
                    .await?
            } else if username.is_empty() {
                bot.send_message(msg.chat.id, format!("Please specify a username."))
                    .await?
            } else {
                let user_id = msg.from().unwrap().id.to_string();
                match req(
                    chan,
                    StateRequest::InsertUser {
                        user_id: user_id.clone(),
                        ea_id: username.clone(),
                    },
                )
                .await
                {
                    StateResponse::Ok => {
                        info!("bind {user_id} with {username}");
                        bot.send_message(msg.chat.id, format!("Bind {user_id} with {username}."))
                            .await?
                    }
                    _ => bot.send_message(msg.chat.id, "Failed to bind").await?,
                }
            }
        }
        Command::Unbind => {
            if !msg.chat.id.is_user() {
                bot.send_message(msg.chat.id, format!("Please PM to unbind user."))
                    .await?
            } else {
                let user_id = msg.from().unwrap().id.to_string();
                match req(
                    chan,
                    StateRequest::DeleteUser {
                        user_id: user_id.clone(),
                    },
                )
                .await
                {
                    StateResponse::Ok => {
                        info!("unbind {user_id}");
                        bot.send_message(msg.chat.id, format!("Unbind {user_id}."))
                            .await?
                    }
                    _ => bot.send_message(msg.chat.id, "Failed to unbind").await?,
                }
            }
        }
    };

    Ok(())
}
