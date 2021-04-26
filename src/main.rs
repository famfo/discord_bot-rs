extern crate systemstat;

use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::model::channel::Message;
use serenity::model::guild::Member;
use serenity::model::id::UserId;
//use serenity::model::error::Error;
//use serenity::prelude::SerenityError::Error;
use serenity::model::prelude::guild;
use serenity::framework::standard::{
    StandardFramework,
    CommandResult,
    macros::{
        command,
        group
    }
};

//use std::env;
use std::fs;
use std::fmt;
use std::fmt::Debug;
//use json::parse;

use std::thread;
use std::time::Duration;
use systemstat::{System, Platform, saturating_sub_bytes};

#[group]
#[commands(duden, the_missile, poll, bot_info, kick)]

struct General;

struct Handler;

struct EmbedAuthor<Author> {
    embed_author: Option<Author>
}

impl <Author: Debug> std::fmt::Display for EmbedAuthor<Author> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Poll by: {:?}", self.embed_author)
    }
}

#[async_trait]
impl EventHandler for Handler {

}

#[tokio::main]
async fn main() {
    let framework=StandardFramework::new()
        .configure(|c| c.prefix("$")) 
        .group(&GENERAL_GROUP);

    //let token_tmp=fs::read_to_string("external/token");
    //let token=env::var(fs::read_to_string("external/token")).expect("token");

    let mut client=Client::builder("ODMwMTQzNTI2MzM0ODkwMDA0.YHCZZw.dHDt7OroeagSX88LYEukB7A5PoE")
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    if let Err(why)=client.start().await {
        println!("Error: {:?}", why);
    }
}


//fun commands
#[command]
async fn foo(ctx: &Context, msg: &Message) -> CommandResult {
    //println!("$foo_rs, {:?}", msg.author);
    //println!("{:?}", msg.content);
    
    let args=msg.content[4..].split_once(" ").unwrap();

    //println!("{:?}", args.1);

    let message=msg.channel_id.send_message(&ctx, |m| m.content(&args.1)).await;
    if let Err(why)=message{println!("Error sending message: {}", why);}
    
    Ok(())
}

#[command]
async fn duden(ctx: &Context, msg: &Message) -> CommandResult {
    println!("$duden, {:?}", msg.author);
    let duden=fs::read_to_string("external/duden")
        .expect("Something went wrong reading the file");

    //msg.reply(ctx, &duden).await?;

    let message=msg.channel_id.send_message(&ctx, |m| m.content(&duden)).await;
    if let Err(why)=message{println!("Error sending message: {}", why);}

    Ok(())
}

#[command]
async fn the_missile(ctx: &Context, msg: &Message) -> CommandResult {
    println!("$the_missile, {:?}", msg.author);
    let the_missile=fs::read_to_string("external/theMissile")
        .expect("Something went wrong reading the file");
    
    let embed=msg.channel_id.send_message(&ctx, |m|{
        m.embed(|e|{
            e.title("The Missile:")
             .description(&the_missile)
             .footer(|f|{
                f.icon_url("https://docs.rs/rust-logo-20210302-1.52.0-nightly-35dbef235.png")
                .text("Coded it rust-lang")
            })
        })
    }).await;
    if let Err(why)=embed{println!("Error sending message: {}", why);}

    Ok(())
}

#[command]
async fn poll(ctx: &Context, msg: &Message) -> CommandResult {
    println!("$poll, {:?}", msg.author);
    let args=msg.content[2..].split_once(" ").unwrap();  

    let embed=msg.channel_id.send_message(&ctx, |m|{
        m.embed(|e| {
            e.title("Poll:")
            .description(args.1)
            .footer(|f|{
                f.icon_url("https://docs.rs/rust-logo-20210302-1.52.0-nightly-35dbef235.png")
                .text("Coded it rust-lang")
            })
        })
    }).await;
    
    let poll=embed.unwrap();
    poll.react(&ctx,'✅').await.unwrap();
    poll.react(&ctx,'❌').await.unwrap();

    Ok(())
}

#[command]
async fn bot_info(ctx: &Context, msg: &Message) -> CommandResult {
    let sys = System::new();
    let message=msg.channel_id.send_message(&ctx, |m| {
        m.content("Collecting system information...")
    }).await;
    match sys.cpu_load_aggregate() {
        Ok(cpu)=> {
            thread::sleep(Duration::from_secs(1));

            let cpu = cpu.done().unwrap();
            let cpu_usage_user=cpu.user*100.0;
            let cpu_usage_system=cpu.system*100.0;

            match sys.memory() {
                Ok(mem)=> {
                    let mem_usage=saturating_sub_bytes(mem.total, mem.free);
                    let embed=msg.channel_id.send_message(&ctx, |m|{
                        m.embed(|e| {
                            e.title("Yet Another Moderation Bot#3550")
                            .color(109632)
                            .description("Bot made by famfo#0227
                                          Yea I made this bot because I was bored.")
                            .field("CPU usage user (in %):", cpu_usage_user, false)    
                            .field("CPU usage system (in %):", cpu_usage_system, false)
                            .field("RAM usage", mem_usage, false)
                            .footer(|f|{
                                f.icon_url("https://docs.rs/rust-logo-20210302-1.52.0-nightly-35dbef235.png")          
                                .text("Coded it rust-lang")
                            })
                        })
                    }).await;
                    if let Err(why)=embed{println!("Error sending message: {}", why);}
                }
                Err(x) => println!("Memory: error: {}", x)
            }
        }
        Err(x) => println!("CPU load: error: {}", x)
    }
    Ok(())
}


//mod commands
#[command]
async fn kick(ctx: &Context, msg: &Message) -> CommandResult {
    println!("$kick, {:?}", msg.author);
    println!("{:?}", msg.mentions);
        
    //for x in &msg.mentions {
    //    println!("{}", x); 
    //}

    let member=&msg.mentions[0];
    let guild=Some(msg.guild_id);

    println!("{:?}", member1);

    //println!("{:?}", embed_author);

    println!("{:?}", msg.member(&ctx.http).await.unwrap().roles(&ctx.cache).await.unwrap());
    println!("{:?}", msg.guild_id);

    /*if msg.member(&ctx.http).await.unwrap().roles(&ctx.cache).await.unwrap().iter().any(|r| r.permissions.kick_members()) {
        //let member=msg.mentions.await.unwrap().UserId.await.unwrap().iter();
        match guild.kick_with_reason(cache_http.http(), &member, "").await {
            Ok(()) => println!("Successfully kicked member"),
            _ => {},
        }

        let message=msg.channel_id.send_message(&ctx, |m| m.content("User kicked")).await;
        if let Err(why)=message{println!("Error sending message: {}", why);}
    }

    else {
        let message=msg.channel_id.send_message(&ctx, |m| m.content("You dont have premissiones to kick members")).await;
        if let Err(why)=message{println!("Error sending message: {}", why);}
    }*/
    Ok(())
}

 
