#[allow(unused_must_use)]

mod ping;
mod admincmds;
mod erfan;
mod shell;

use substring::Substring;
use erfan::*;
use admincmds::*;
use teloxide::{prelude::*, utils::command::BotCommand};
use crate::shell::sh;

#[derive(BotCommand)]
#[command(rename = "lowercase", description = "available commands:\n")]
enum Cmds
{
    #[command(description = "/kick <kicks a user>")]
    Kick,
    #[command(description="pulls up the help menu")]
    Help,
    #[command(description="check the bot's ping speed")]
    Ping,
    #[command(description="delete the message you reply to")]
    Del,
    #[command(description="get the groups's admins")]
    Admins,
    #[command(description="Pins the message you reply to")]
    Pin,
    #[command(description="Makes the bot leaves the chat")]
    Leave,
    #[command(description="Erfan gsis")]
    GSI,
    #[command(description="excute a bash command")]
    SH,
    #[command(description="get the replied user's info")]
    Info,
}
type Cx = teloxide::prelude::UpdateWithCx<teloxide::prelude::Message>;

async fn do_cmds(cx:Cx,cmd:Cmds) -> ResponseResult<()>
{
    let owner_id = std::env::var("OWNER_ID").expect("owner id not set bitch");
    match cmd
    {
        Cmds::Help => cx.answer(Cmds::descriptions()).send().await.map(|_| ())?,
        Cmds::Kick => kick_user(&cx).await?,
        Cmds::Ping => ping::ping(&cx).await?,
        Cmds::Pin => pin(&cx).await?,
        Cmds::Admins => list_admins(&cx).await?,
        Cmds::Del => del(&cx).await?,
        Cmds::Leave => leave(&cx,owner_id).await?,
        Cmds::SH => sh(&cx).await?,
        Cmds::Info => get_user_info(&cx).await?,
        Cmds::GSI =>
            {
                let val = cx.update.text().unwrap().substring(5,cx.update.text().unwrap().len()).find(" ").unwrap();
                let e = cx.update.text().unwrap().replace("/gsi","");
                let link = e.substring(0,val + 1);
                let mut rom_name:String = String::new();
                rom_name.push_str(e.substring(val + 1,e.len()));
                let gsi = Gsi::new(rom_name.to_string(),link.to_string());
                gsi.make(&cx).await?;
            }
    }
    Ok(())
}
#[tokio::main]
async fn main()
{ run().await; }

async fn run()
{
    teloxide::enable_logging!();
    let bot = teloxide::Bot::from_env();
    teloxide::commands_repl(bot,"gay",do_cmds).await;
}