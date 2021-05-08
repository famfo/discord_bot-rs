extern crate systemstat;

use serenity::{async_trait, model::id::UserId};
use serenity::client::{Client, Context, EventHandler};
use serenity::model::channel::Message;
use serenity::framework::standard::{
    StandardFramework,
    CommandResult,
    macros::{
        command,
        group
    }
};

use std::fs;
use std::fmt;
use std::fmt::Debug;

use std::thread;
use std::time::Duration;
use systemstat::{System, Platform, saturating_sub_bytes};

#[group]
#[commands(duden, the_missile, poll, bot_info, kick, ban, unban)]

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

    //let token_tmp=fs::read_to_string("external/token_dev");
    //let token=env::var(&token_tmp).expect("token");

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
    let args=msg.content[4..].split_once(" ").unwrap();
    let message=msg.channel_id.send_message(&ctx, |m| m.content(&args.1)).await;
    if let Err(why)=message{println!("Error sending message: {}", why);}
    
    Ok(())
}

#[command]
async fn duden(ctx: &Context, msg: &Message) -> CommandResult {
    println!("$duden, {:?}", msg.author);
    let duden=fs::read_to_string("external/duden")
        .expect("Something went wrong reading the file");

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
    if let Err(why)=message{println!("Error sending message: {}", why);}
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
    let member=&msg.mentions[0];
    let guild=msg.guild_id.unwrap();

    if msg.member(&ctx.http).await.unwrap().roles(&ctx.cache).await.unwrap().iter().any(|r| r.permissions.kick_members()) {
        match guild.kick(ctx, member).await {
            Ok(()) => println!("Successfully kicked member"),
            _ => {},
        }
        let message=msg.channel_id.send_message(&ctx, |m| m.content("User kicked")).await;
        if let Err(why)=message{println!("Error sending message: {}", why);}
    }
    else {
        let message=msg.channel_id.send_message(&ctx, |m| m.content("You dont have premissiones to kick members")).await;
        if let Err(why)=message{println!("Error sending message: {}", why);}
    }
    Ok(())
}

#[command]
async fn ban(ctx: &Context, msg: &Message) -> CommandResult {
    let member=&msg.mentions[0];
    let guild=msg.guild_id.unwrap();

    if msg.member(&ctx.http).await.unwrap().roles(&ctx.cache).await.unwrap().iter().any(|r| r.permissions.ban_members()) {
        match guild.ban(&ctx, member, 0).await {
            Ok(()) => println!("Successfully banned member"),
            _ => {},
        }
        let message=msg.channel_id.send_message(&ctx, |m| m.content("User banned")).await;
        if let Err(why)=message{println!("Error sending message: {}", why);}
    }
    else {
        let message=msg.channel_id.send_message(&ctx, |m| m.content("You dont have premissiones to ban members")).await;
        if let Err(why)=message{println!("Error sending message: {}", why);}
    }
    Ok(())
}

#[command]
async fn unban(ctx: &Context, msg: &Message) -> CommandResult {
    let user_id=msg.content[2..].split_once(" ").unwrap();
    let member=UserId(user_id.1.parse::<u64>().unwrap());
    let guild=msg.guild_id.unwrap();

    if msg.member(&ctx.http).await.unwrap().roles(&ctx.cache).await.unwrap().iter().any(|r| r.permissions.ban_members()) {
        match guild.unban(&ctx, member).await {
            Ok(()) => println!("Successfully unbanned member"),
            _ => {},
        }
        let message=msg.channel_id.send_message(&ctx, |m| m.content("User unbanned")).await;
        if let Err(why)=message{println!("Error sending message: {}", why);}
    }
    else {
        let message=msg.channel_id.send_message(&ctx, |m| m.content("You dont have premissiones to unban members")).await;
        if let Err(why)=message{println!("Error sending message: {}", why);}
    }
    Ok(())
}
