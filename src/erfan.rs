#[allow(unused_must_use)]

use teloxide::prelude::*;
use std::env;
use std::process::{Command,Stdio};
use std::io::{BufReader, BufRead};
use teloxide::types::{ChatOrInlineMessage, ParseMode, InlineKeyboardButton, InlineKeyboardMarkup};
use std::fs;
use substring::Substring;

type Cx = UpdateWithCx<Message>;

pub struct Gsi
{
    name:String,
    link:String,
    args:String
}

impl Gsi
{
    pub fn new(name:String,link:String,args:String) -> Self
    {
        Gsi
        {
            name:name,
            link:link,
            args:args
        }
    }
    pub async fn make(&self,bot:&Cx) -> ResponseResult<()>
    {
        if bot.update.text().unwrap().contains("cancel")
        {
            Command::new("sh")
                .arg("-c")
                .arg("kill -TERM -- -$(ps ax | grep url2GSI.sh | grep -v grep | awk '{print $1;}')") // kanged from bot3+t
                .output()
                .expect("bruh");
        }
        if self.link.is_empty() || self.name.is_empty()
        {
            bot.answer("usage:\n/gsi <link> <rom_name>").send().await?;
            return Ok(());
        }
        let channel_id = std::env::var("CHANNEL").expect("channel id not set");
        let owner_id = env::var("OWNER_ID").expect("OWNER_ID has not been set!");
        let mut logs = String::new();
        let mut done:bool = false;

        if bot.update.from().unwrap().id.to_string() != owner_id
        {
            bot.answer("you're not my owner.").send().await?;
            return Ok(());
        }
        //here we excute the command to make and upload the gsi
        // let gsi = queue.lock().unwrap().get(0)
        // {
        //     Some(e) => e,
        //     _ => Gsi::new("bruh".to_string(),"bruh".to_string())
        // };
        let rom_name = &self.name;
        let rom_link = &self.link;
        let cmd = format!("cd ErfanGSIs;sudo ./url2GSI.sh {0}{1} {2};cd ../src;python3 zip_upload.py {1}", rom_link, rom_name,self.args);
        let out = Command::new("sh")
            .arg("-c")
            .arg(cmd.as_str())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap()
            .stdout
            .expect("bruh");
        &logs.push_str("gsi making started\n");
        let msg = bot.answer(&logs).send().await?;

        // here starts the bufreader which will get us some live ouput of the gsi
        let reader = BufReader::new(out);
        let mut need_logs = true;
        for line in reader.lines().filter_map(|line| line.ok())
        {
            if line != ""
            {
                if line.contains("Downloading")
                { need_logs = false }
                if line.contains("Download complete")
                { need_logs = true }
                if need_logs
                {
                    &logs.push_str("<code>\n");
                    &logs.push_str(line.as_str());
                    &logs.push_str("</code>\n");
                    if line.contains("zip error")
                    {
                        bot.answer("no output found! gsi is probably not treble supported!").send().await?;
                        return Ok(());
                    }
                    bot.bot.edit_message_text
                    (
                        ChatOrInlineMessage::Chat {
                            chat_id: bot.update.chat.id.into(),
                            message_id: msg.id
                        },
                        &logs
                    ).parse_mode(ParseMode::HTML).send().await?;
                }
                println!("{}",&logs);
            }
            if line.contains("GSI done on:")
            {
                done = true;
                need_logs = false;
            }
        }
        if done
        {
            bot.bot.edit_message_text(ChatOrInlineMessage::Chat
                                      {
                                          chat_id: bot.update.chat.id.into(),
                                          message_id: msg.id
                                      },
                                      "process completed").parse_mode(ParseMode::HTML).send().await?;

            // here we match for each test file to get the output of it otherwise we will just put "bruh"
            let mut ab = match fs::read_to_string("ab.txt")
            {
                Ok(x) => x,
                _ => String::from("bruh")
            };
            let mut aonly = match fs::read_to_string("aonly.txt")
            {
                Ok(x) => x,
                _ => String::from("bruh")
            };
            let build_info = match fs::read_to_string("bruh.txt")
            {
                Ok(e) => e,
                _ => String::from("bruh")
            };
            // here we start a clean up so there would be more space for more gsis
            Command::new("sh")
                .arg("-c")
                .arg("rm -rf *.txt;sudo rm -rf ErfanGSIs/output/*")
                .output()
                .expect("bruh");

            //here we substring the link to get the download link only and remove the "Download Link:" text

            ab = ab.substring(14, 69420).to_string();
            aonly = aonly.substring(14, 69420).to_string();
            let mut reply = String::new();
            reply.push_str(
                format!(
                    "{} - GSI\ninformation:\n\n<code>{}</code>\n\nAB:{}\naonly:{}"
                    , rom_name.as_str(), build_info.as_str(), ab.as_str(), aonly.as_str()).as_str());
            let buttons = vec![InlineKeyboardButton::url(
                "ErfanGSIs-VelanGSIs".to_string(), "https://github.com/Velosh/ErfanGSIs-VelanGSIs".to_string()),
                               InlineKeyboardButton::url("Rust-5X".to_string(), "https://github.com/aktham3210/Rust-5X".to_string())
            ];
            let markup = InlineKeyboardMarkup::default()
                .append_row(buttons);
            bot.bot.send_message(channel_id,reply).parse_mode(ParseMode::HTML).disable_web_page_preview(true).reply_markup(markup).send().await?;
            bot.answer("completed").send().await?;
        } else {
            bot.answer("gsi failed").send().await?;
        }
        log::info!("excuted function: /gsi");
        Ok(())
    }
}
