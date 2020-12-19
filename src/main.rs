use std::env;
use reqwest;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::{Client, Context, EventHandler},
};
async fn get_quote() -> String {
    let res = reqwest::get("https://inspirobot.me/api?generate=true").await.unwrap();
    if res.status() == reqwest::StatusCode::OK {
        res.text().await.unwrap()
    }else {
        "A wise man once said : ERROR 404. Could not get a quote".to_string()
    }
    
}

struct Handler;
#[async_trait]
impl EventHandler for Handler {
    
    async fn message(&self, ctx : Context, message : Message){

       /*  let res = match message.content.to_ascii_lowercase().as_str() {
            "!ping" => "pong!".to_string(),
            "!quote" => get_quote().await,
            "!help" => "Hi there!\n I am S.P.A.M, the simply, pleasing, automatic moderator.\n Commands : \t!ping \n\t!quote\n Use them to find out what they do!".to_string(),
            _ => "Not yet implemented :) Use '!help' to get all the commands".to_string()
        };
        if let Err(why) = message.channel_id.say(&ctx.http,res).await {
            println!("Error sending messsage {:?}",why);
        } */

       if message.content.to_ascii_lowercase() == "!ping"{
            if let Err(why) = message.channel_id.say(&ctx.http,"pong!").await {
                println!("Error sending messsage {:?}",why);
            }
       }
       else if message.content.to_ascii_lowercase() == "!quote" {
           if let Err(why) = message.channel_id.say(&ctx.http,get_quote().await).await {
               println!("Error sending quote {:?}",why)
           }
       }
       else if message.content.to_ascii_lowercase() == "!help" {
        if let Err(why) = message.channel_id.say(&ctx.http,"Hi there!\nI am S.P.A.M, the simply, pleasing, automatic moderator.\n Commands : \n\t!ping \n\t!quote\n Use them to find out what they do!").await {
            println!("Error sending quote {:?}",why)
            }
        }
        else if message.content.starts_with("!") {
            if let Err(why) = message.channel_id.say(&ctx.http,"Not yet implemented :) Use '!help' to get all the commands").await {
                println!("Error sending quote {:?}",why)
                }
        }
    }
    async fn ready(&self,_: Context, ready: Ready){
        println!("{} is connected",ready.user.name)
    }
}
#[tokio::main]
async fn main() {
    let mut client = Client::builder(env::var("DISCORDTOKEN").unwrap()).event_handler(Handler).await.
    expect("Unable to set up client");
    if let Err(why) = client.start().await {
        println!("Client error {:?}",why)
    }
}
