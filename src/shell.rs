#[allow(unused_must_use)]

use std::process::Command;
use teloxide::prelude::*;
use teloxide::types::{ParseMode, ChatOrInlineMessage};

type Cx = UpdateWithCx<Message>;

pub async fn sh(cx:&Cx) -> ResponseResult<()>
{
    let owner_id = std::env::var("OWNER_ID").expect("bruh owner_id not set");
    if cx.update.from().unwrap().id.to_string() == owner_id
    {
        let mut cmd = cx.update.text().unwrap().replace("/sh", "");
        cmd = cmd.replace("\"", "\\\"");
        let message = cx.answer(format!("<b>${}\n</b>", cmd)).parse_mode(ParseMode::HTML).send().await?;
        let output = Command::new("sh")
            .arg("-c")
            .arg(cmd.as_str())
            .output()
            .expect("bruh");
        let reply = format!("<code>{}</code>", String::from_utf8_lossy(&output.stdout));

        cx.bot.edit_message_text(
            ChatOrInlineMessage::Chat
            {
                chat_id:cx.update.chat.id.into(),
                message_id:message.id
            },
            reply
        )
            .parse_mode(ParseMode::HTML)
            .send()
            .await?;
    }
    else {cx.answer("bruh you're not my owner :P").send().await?;}
    Ok(())
}