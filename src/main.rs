use teloxide::{prelude2::*, types::InputFile};
use regex::*;
use tiktok_uploader_telegram::tiktok::*;


#[tokio::main]
async fn main() {
    teloxide::enable_logging!();
    log::info!("Starting dices_bot...");

    let bot = Bot::from_env().auto_send();

    teloxide::repls2::repl(bot, |message: Message, bot: AutoSend<Bot>| async move {
        let regex = Regex::new(r"http(s|)://.*(tiktok).com[^\s]*").unwrap();
        let message_text = message.text().unwrap();
        if regex.is_match(message_text) {
            let resp= Response::new(message_text).await.unwrap();
            resp.download_video().await;
            let file = InputFile::file(resp.get_file_name());
            let content = format!("Author: {}\nDescription: {}\nDuration: {}\nDate uploaded: {}", 
                resp.aweme_detail.author.unique_id, 
                resp.get_description(), 
                resp.get_duration(), 
                resp.get_date_created());

            bot.send_message(message.chat_id(), content).await?;
            bot.send_video(message.chat_id(), file).await?;
            resp.delete_video().await;

        } else {
            bot.send_message(message.chat_id(), "Not a tiktok link.").await?;
        }
        respond(())
    })
    .await;
}
