use crate::backend::{req, ProducerChan, StateRequest, StateResponse};
use teloxide::{
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup},
};

pub fn make_vehicle_keyboard() -> InlineKeyboardMarkup {
    let vehicle_type = vec!["Tank", "Plane", "Other"];
    let vehicle_button = vehicle_type
        .iter()
        .map(|t| InlineKeyboardButton::callback(t.to_owned(), t.to_owned()))
        .collect::<Vec<InlineKeyboardButton>>();

    InlineKeyboardMarkup::new(vec![vehicle_button])
}

pub async fn get_ea_id(
    chan: ProducerChan,
    bot: &Bot,
    msg: &Message,
    username: String,
) -> Result<String, ()> {
    if username.is_empty() {
        match req(
            chan.clone(),
            StateRequest::QueryUser {
                user_id: msg.from().unwrap().id.to_string(),
            },
        )
        .await
        {
            StateResponse::EaUser(u) => Ok(u),
            _ => {
                bot.send_message(
                    msg.chat.id,
                    "Failed to get your EA username, please set it with /bind",
                )
                .await
                .map_err(|_| ())?;
                Err(())
            }
        }
    } else {
        Ok(username)
    }
}
