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
    log::info!("String tiktok uploader bot...");

    let bot = Bot::from_env().auto_send();

    teloxide::repls2::repl(bot, |message: Message, bot: AutoSend<Bot>| async move {
        let message_text = message.text().unwrap();
        if REGEX.is_match(message_text) {

            let resp=  Response::new(message_text).await.ok();
            let response : Response;

            if let None = resp {
                bot.send_message(message.chat_id(), "Your tiktok link does not lead to anything.").await?;
                return respond(());
            }
            else {
                response = resp.unwrap();
            }
            let err = response.download_video().await.ok();
            if let None = err {
                bot.send_message(message.chat_id(), "Could not download the specified video, please try again later.").await?;
                return respond(());
            }
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
            bot.send_message(message.chat_id(), "Not a tiktok link.").await?;
        }
        respond(())
    })
    .await;
}
