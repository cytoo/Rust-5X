#[allow(unused_variables)]

use std::borrow::Borrow;
use teloxide::prelude::*;
use teloxide::types::{ParseMode, ChatOrInlineMessage};

type Cx = UpdateWithCx<Message>;

pub async fn list_admins(bot:&Cx) -> ResponseResult<()>
{
    if bot.update.chat.is_private()
    {
        bot.answer("`bruh use this command in a chat`").parse_mode(ParseMode::MarkdownV2).send().await?;
        return Ok(());
    }
    let admins = bot.bot.get_chat_administrators(bot.update.chat.id).send().await?;
        let msg = bot.answer("fetching admins...").send().await?;
        let mut reply = String::new();
        reply.push_str("*Admins*: ");
        for admin in admins
        {
            reply.push_str(format!("\n• *[{0}](t.me/{2}): id: `{1}`\n", admin.user.first_name, admin.user.id,admin.user.username.unwrap().as_str()).borrow());
        }
        bot.bot.edit_message_text(
            ChatOrInlineMessage::Chat {
                chat_id:bot.update.chat.id.into(),
                message_id:msg.id
            },
            reply
        )
            .parse_mode(ParseMode::MarkdownV2).disable_web_page_preview(true).send().await?;
    log::info!("excuted function: /admins");
    Ok(())
}


pub async fn leave(bot:&Cx,owner_id:String) -> ResponseResult<()>
{
    if bot.update.from().unwrap().id.to_string() == owner_id
    {
        bot.bot.leave_chat(bot.update.chat.id).send().await?;
    }
    else {bot.answer("bruh you're not my owner").send().await?;}
    log::info!("excuted function: /pin");
    Ok(())
}

pub async fn pin(bot:&Cx) -> ResponseResult<()>
{
    if bot.update.chat.is_private()
    {
        bot.reply_to("bruh this isn't even a group").send().await?;
        return Ok(());
    }
    if let Some(rep) = bot.update.reply_to_message()
    {
        bot.bot.pin_chat_message(bot.update.chat.id,rep.id).send().await?;
    }
    log::info!("excuted function: /pin");
    Ok(())
}

pub async fn del(bot:&Cx) -> ResponseResult<()>
{
    bot.delete_message().send().await?;
    if let Some(rep) = bot.update.reply_to_message()
    {
        bot.bot.delete_message(bot.update.chat.id,rep.id).send().await?;
    }
    log::info!("excuted function: /del");
    Ok(())
}

pub async fn kick_user(cx:&Cx) -> ResponseResult<()> {
    if cx.update.chat.is_private()
    {
        let _e = cx.answer("`bruh this isn't a group`").parse_mode(ParseMode::MarkdownV2).send().await?;
        return Ok(());
    }
    match cx.update.reply_to_message() {
        Some(mes) => {
            cx.bot.unban_chat_member(cx.update.chat_id(), mes.from().unwrap().id).send().await?;
        },
        None => {
            cx.reply_to("`Use this command in reply to another message`").send().await?;
        }
    };
    log::info!("excuted function: /kick");
    Ok(())
}

pub async fn get_user_info(cx:&Cx) -> ResponseResult<()>
{
     match cx.update.reply_to_message()
     {
         Some(reply) =>
             {
                 let user = reply.from().unwrap();
                 let last_name = match &user.last_name
                 {
                   Some(name) => name,
                     _ => "null"
                 };
                 let lang_code = match &user.language_code
                 {
                     Some(x) => x,
                     _ => "null"
                 };
                 let info = format! {
                     "• This chat's id: <code>{}</code>\n• <b>first name</b>: {}\n• <b>last name</b>: <code>{}</code>\n id: <code>{}</code>\nLanguage code: {}\nis_bot: {}",
                     cx.update.chat.id,
                     user.first_name,
                     last_name,
                     user.id,
                     lang_code,
                     user.is_bot
                 };
                 cx.reply_to(info).parse_mode(ParseMode::HTML).send().await?;
             },
         None => {cx.reply_to("reply to someone!").send().await?;}
     }
    Ok(())
}