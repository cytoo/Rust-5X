use teloxide::prelude::*;
use teloxide::types::{ParseMode, ChatOrInlineMessage};

pub async fn ping(bot:&UpdateWithCx<Message>) -> ResponseResult<()>
{
    let mut watch = stopwatch::Stopwatch::new();
    &watch.start();
    let e = bot.answer("`calculating...`").parse_mode(ParseMode::MarkdownV2).send().await?;
    &watch.stop();
    bot.bot.edit_message_text(ChatOrInlineMessage::Chat {
        chat_id:bot.update.chat.id.into(),
        message_id:e.id
    }, format!("`ping={}`",watch.elapsed_ms())).parse_mode(ParseMode::MarkdownV2).send().await?;
    log::info!("excuted function: /ping");
    Ok(())
}