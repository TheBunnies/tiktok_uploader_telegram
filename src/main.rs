use teloxide::{prelude2::*, types::InputFile};
use regex::*;
use tiktok_uploader_telegram::tiktok::*;

use lazy_static::lazy_static;
lazy_static! {
    static ref REGEX: Regex = Regex::new(r"http(s|)://.*(tiktok).com[^\s]*").unwrap();
}

#[tokio::main]
async fn main() {
    teloxide::enable_logging!();
    log::info!("Starting tiktok uploader bot...");

    let bot = Bot::from_env().auto_send();
    teloxide::repls2::repl(bot, |message: Message, bot: AutoSend<Bot>| async move {
        let message_text = message.text().unwrap_or_default();
        if REGEX.is_match(message_text) {

            let resp=  Response::new(message_text).await.ok();
            let response : Response;

            if let None = resp {
                let user = message.from().unwrap();
                let last_name = user.last_name.to_owned().unwrap_or(String::new());
                log::info!("Rejected request `Tiktok link does not lead to anything`: {} From user: {} {}/{}", message_text, user.first_name, last_name, user.id);
                bot.send_message(message.chat_id(), "Your tiktok link does not lead to anything.").await?;
                return respond(());
            }
            else {
                response = resp.unwrap();
            }
            let err = response.download_video().await.ok();
            if let None = err {
                let user = message.from().unwrap();
                let last_name = user.last_name.to_owned().unwrap_or(String::new());
                log::info!("Couldn't download the video: {} From user: {} {}/{}", message_text, user.first_name, last_name, user.id);
                bot.send_message(message.chat_id(), "Could not download the specified video, please try again later.").await?;
                return respond(());
            }
            let user = message.from().unwrap();
            let last_name = user.last_name.to_owned().unwrap_or(String::new());
            log::info!("Now processing: {} From user: {} {}/{}", message_text, user.first_name, last_name, user.id);
            let file = InputFile::file(response.get_file_name());
            let content = format!("Author: {}\nDescription: {}\nDuration: {}\nDate uploaded: {}", 
            response.aweme_detail.author.unique_id, 
                response.get_description(), 
                response.get_duration(), 
                response.get_date_created());

            bot.send_message(message.chat_id(), content).await?;
            bot.send_video(message.chat_id(), file).await?;
            response.delete_video().await.expect("Could not delete the video... Starting to panic!!!");

        } else {
            let user = message.from();
            if let Some(state) = user {
                let last_name = state.last_name.to_owned().unwrap_or(String::new());
                log::info!("Rejected request `Not a tiktok link`: {} From user: {} {}/{}", message_text, state.first_name, last_name, state.id);
                bot.send_message(message.chat_id(), "Not a tiktok link.").await?;
            }
        }
        respond(())
    })
    .await;
}
