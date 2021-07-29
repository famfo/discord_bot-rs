/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

extern crate systemstat;

use serenity::client::{Client, Context, EventHandler};
use serenity::framework::standard::{
    macros::{command, group},
    CommandResult, StandardFramework,
};
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::{async_trait, model::id::UserId};

use dotenv;
use std::env;
use std::fs;
use std::thread;
use std::time::Duration;
use systemstat::{saturating_sub_bytes, Platform, System};

#[group]
#[commands(
    ban,
    bot_info,
    duden,
    foo,
    help,
    kick,
    mute,
    poll,
    the_missile,
    unban,
    unmute
)]

struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("$"))
        .group(&GENERAL_GROUP);

    dotenv::dotenv().ok();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the enviroment");

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    if let Err(e) = client.start().await {
        println!("Error: {:?}", e);
    }
}

/*
 * Moderation command
 * ban: ban a member of a guild
 */
#[command]
async fn ban(ctx: &Context, msg: &Message) -> CommandResult {
    let member = &msg.mentions[0];
    let guild = msg.guild_id.unwrap();

    if msg
        .member(&ctx.http)
        .await
        .unwrap()
        .roles(&ctx.cache)
        .await
        .unwrap()
        .iter()
        .any(|r| r.permissions.ban_members())
    {
        if let Ok(()) = guild.ban(&ctx, member, 0).await {
            println!("Successfully banned member")
        }
        let message = msg
            .channel_id
            .send_message(&ctx, |m| m.content("User banned"))
            .await;
        if let Err(e) = message {
            println!("Error sending message: {}", e)
        }
    } else {
        let message = msg
            .channel_id
            .send_message(&ctx, |m| {
                m.content("You dont have premissiones to ban members")
            })
            .await;
        if let Err(e) = message {
            println!("Error sending message: {}", e)
        }
    }
    Ok(())
}

/*
 * General information
 * bot_info: information about the bot (wow)
 * TODO: cleanup code
 */
#[command]
async fn bot_info(ctx: &Context, msg: &Message) -> CommandResult {
    let sys = System::new();
    let message = msg
        .channel_id
        .send_message(&ctx, |m| m.content("Collecting system information..."))
        .await;
    if let Err(e) = message {
        println!("Error sending message: {}", e);
    }
    match sys.cpu_load_aggregate() {
        Ok(cpu) => {
            thread::sleep(Duration::from_secs(1));

            let cpu = cpu.done().unwrap();
            let cpu_usage_user = cpu.user * 100.0;
            let cpu_usage_system = cpu.system * 100.0;

            match sys.memory() {
                Ok(mem) => {
                    let mem_usage = saturating_sub_bytes(mem.total, mem.free);
                    let embed = msg.channel_id.send_message(&ctx, |m|{
                         m.embed(|e| {
                             e.title("Yet Another Moderation Bot#3550")
                              .color(109_632)
                              .description("Bot made by famfo#0227
                                           Yea I made this bot because I was bored.")
                              .field("CPU usage user (in %):", cpu_usage_user, false)
                              .field("CPU usage system (in %):", cpu_usage_system, false)
                              .field("RAM usage", mem_usage, false)
                              .field("License", "This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.", false)
                              .field("Source Code", "https://github.com/famfo/discord_bot-rs", false)
                              .footer(|f|{
                                    f.icon_url("https://docs.rs/rust-logo-20210302-1.52.0-nightly-35dbef235.png")
                                     .text("Coded it rust-lang")
                             })
                         })
                     }).await;
                    if let Err(e) = embed {
                        println!("Error sending message: {}", e);
                    }
                }
                Err(x) => println!("Memory: error: {}", x),
            }
        }
        Err(x) => println!("CPU load: error: {}", x),
    }
    Ok(())
}

/*
 * Fun command
 * duden: tell someone to speak german
 */
#[command]
async fn duden(ctx: &Context, msg: &Message) -> CommandResult {
    let duden =
        fs::read_to_string("external/duden").expect("Something went wrong reading the file");

    let message = msg
        .channel_id
        .send_message(&ctx, |m| m.content(&duden))
        .await;
    if let Err(e) = message {
        println!("Error sending message: {}", e);
    }

    Ok(())
}

/*
 * Fun comand
 * foo: returns what you typed
 */
#[command]
async fn foo(ctx: &Context, msg: &Message) -> CommandResult {
    let args = msg.content[4..].split_once(" ").unwrap();
    let message = msg
        .channel_id
        .send_message(&ctx, |m| m.content(&args.1))
        .await;
    if let Err(e) = message {
        println!("Error sending message: {}", e);
    }

    Ok(())
}

/*
 * General information
 * help: shows all available commands
 */
#[command]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    let message = msg.channel_id.send_message(&ctx, |m|{
        m.embed(|e|{
            e.title("Commands")
             .description("All available commands:")
             .field("$ban", "Bans the mentioned user.", false)
             .field("$bot_info", "General information about the bot.", false)
             .field("$duden", "Tell someone to speak german!", false)
             .field("$foo", "Returns what you just typed.", false)
             .field("$help", "Shows this help.", false)
             .field("$kick", "Kicks the mentioned user.", false)
             .field("$mute", "Mutes a member.", false)
             .field("$poll", "Starts a poll.", false)
             .field("$the_missile", "The missile knows where it is...", false)
             .field("$unban", "Unbans a user by their user ID.", false)
             .field("$unmute", "Unmutes a member.", false)
             .field("License", "This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.", false)
             .field("Source Code", "https://github.com/famfo/discord_bot-rs", false)
             .field("Issues", "If you have questiones, encouter bugs or have feature requests, considering opening an issue on github", false)
             .footer(|f|
                f.icon_url("https://docs.rs/rust-logo-20210302-1.52.0-nightly-35dbef235.png")
                 .text("Coded it rust-lang")
             )
        })
    }).await;

    if let Err(e) = message {
        println!("Error sending message: {}", e);
    }

    Ok(())
}

/*
 * Moderation command
 * kick: kick a member of a guild
 */
#[command]
async fn kick(ctx: &Context, msg: &Message) -> CommandResult {
    let member = &msg.mentions[0];
    let guild = msg.guild_id.unwrap();

    if msg
        .member(&ctx.http)
        .await
        .unwrap()
        .roles(&ctx.cache)
        .await
        .unwrap()
        .iter()
        .any(|r| r.permissions.kick_members())
    {
        if let Ok(()) = guild.kick(ctx, member).await {
            println!("Successfully kicked member")
        }
        let message = msg
            .channel_id
            .send_message(&ctx, |m| m.content("User kicked"))
            .await;
        if let Err(e) = message {
            println!("Error sending message: {}", e)
        }
    } else {
        let message = msg
            .channel_id
            .send_message(&ctx, |m| {
                m.content("You dont have premissiones to kick members")
            })
            .await;
        if let Err(e) = message {
            println!("Error sending message: {}", e);
        }
    }
    Ok(())
}

/*
 * Moderation command
 * mute: mute a member of a guild
 * TODO: create mute role if it does't exist
 */
#[command]
async fn mute(ctx: &Context, msg: &Message) -> CommandResult {
    if msg
        .member(&ctx.http)
        .await
        .unwrap()
        .roles(&ctx.cache)
        .await
        .unwrap()
        .iter()
        .any(|r| r.permissions.manage_roles())
    {
        let user_id = &msg.mentions[0];

        // Get the guild the member is in
        if let Some(guild) = msg.guild_id.unwrap().to_guild_cached(&ctx).await {
            // Get the muted role by it's name
            if let Some(role) = guild.role_by_name("Muted") {
                // Assign the mute role to the member
                let mute = guild
                    .member(&ctx, user_id)
                    .await
                    .unwrap()
                    .add_role(&ctx, role.id)
                    .await;
                if let Err(e) = mute {
                    println!("Error muting member: {}", e);
                }

                let message = msg
                    .channel_id
                    .send_message(&ctx, |m| m.content("User muted"))
                    .await;
                if let Err(e) = message {
                    println!("Error sending message {}", e);
                }
            } else {
                let message = msg
                    .channel_id
                    .send_message(&ctx, |m| {
                        m.content("Mute role not found, please create a \"Muted\" role")
                    })
                    .await;
                if let Err(e) = message {
                    println!("Error sending message: {}", e);
                }
            }
        }
    }
    Ok(())
}

/*
 * Fun command
 * poll: start a new poll
 */
#[command]
async fn poll(ctx: &Context, msg: &Message) -> CommandResult {
    if msg.content == "$poll" {
        let empty_message = msg
            .channel_id
            .send_message(&ctx, |m| m.content("No topic for the poll."))
            .await;

        if let Err(e) = empty_message {
            println!("Error sending message: {}", e);
        }
    } else {
        let args = msg.content[2..].split_once(" ").unwrap();
        let embed = msg
            .channel_id
            .send_message(&ctx, |m| {
                m.embed(|e| {
                    e.title("Poll:").description(args.1).footer(|f| {
                        f.icon_url(
                            "https://docs.rs/rust-logo-20210302-1.52.0-nightly-35dbef235.png",
                        )
                        .text("Coded it rust-lang")
                    })
                })
            })
            .await;

        let poll = embed.unwrap();
        poll.react(&ctx, '✅').await.unwrap();
        poll.react(&ctx, '❌').await.unwrap();
    }

    Ok(())
}

/*
 * Fun command
 * the_missile: The missile knows where it is...
 */
#[command]
async fn the_missile(ctx: &Context, msg: &Message) -> CommandResult {
    println!("$the_missile, {:?}", msg.author);
    let the_missile =
        fs::read_to_string("external/theMissile").expect("Something went wrong reading the file");

    let embed = msg
        .channel_id
        .send_message(&ctx, |m| {
            m.embed(|e| {
                e.title("The Missile:")
                    .description(&the_missile)
                    .footer(|f| {
                        f.icon_url(
                            "https://docs.rs/rust-logo-20210302-1.52.0-nightly-35dbef235.png",
                        )
                        .text("Coded it rust-lang")
                    })
            })
        })
        .await;
    if let Err(e) = embed {
        println!("Error sending message: {}", e);
    }

    Ok(())
}

/*
 * Moderation command
 * unban: unban a member using it's userID
 * TODO: make this actually useable
 */
#[command]
async fn unban(ctx: &Context, msg: &Message) -> CommandResult {
    let user_id = msg.content[2..].split_once(" ").unwrap();
    let user_id = UserId(user_id.1.parse::<u64>().unwrap());
    let guild = msg.guild_id.unwrap();

    if msg
        .member(&ctx.http)
        .await
        .unwrap()
        .roles(&ctx.cache)
        .await
        .unwrap()
        .iter()
        .any(|r| r.permissions.ban_members())
    {
        if let Ok(()) = guild.unban(&ctx, user_id).await {
            println!("Successfully unbanned member")
        }
        let message = msg
            .channel_id
            .send_message(&ctx, |m| m.content("User unbanned"))
            .await;
        if let Err(e) = message {
            println!("Error sending message: {}", e);
        }
    } else {
        let message = msg
            .channel_id
            .send_message(&ctx, |m| {
                m.content("You dont have premissiones to unban members")
            })
            .await;
        if let Err(e) = message {
            println!("Error sending message: {}", e);
        }
    }
    Ok(())
}

/*
 * Moderation command
 * unmute: unmutes a member of a guild
 */
#[command]
async fn unmute(ctx: &Context, msg: &Message) -> CommandResult {
    if msg
        .member(&ctx.http)
        .await
        .unwrap()
        .roles(&ctx.cache)
        .await
        .unwrap()
        .iter()
        .any(|r| r.permissions.manage_roles())
    {
        let user_id = &msg.mentions[0];
        let guild = msg.guild_id.unwrap();

        // See if the member has the muted role
        if guild
            .member(&ctx, user_id)
            .await
            .unwrap()
            .roles(&ctx)
            .await
            .unwrap()
            .iter()
            .any(|r| matches!(r.name.clone().as_str(), "Muted"))
        {
            // Get guild the member is in
            if let Some(guild) = msg.guild_id.unwrap().to_guild_cached(&ctx).await {
                // Get the muted role by it's name
                if let Some(role) = guild.role_by_name("Muted") {
                    // Remove the muted role from the member
                    let unmute = guild
                        .member(&ctx, user_id)
                        .await
                        .unwrap()
                        .remove_role(&ctx, role.id)
                        .await;
                    if let Err(e) = unmute {
                        println!("Error muting member: {}", e);
                    }

                    let message = msg
                        .channel_id
                        .send_message(&ctx, |m| m.content("User unmuted"))
                        .await;
                    if let Err(e) = message {
                        println!("Error sending message {}", e);
                    }
                }
            }
        } else {
            let message = msg
                .channel_id
                .send_message(&ctx, |m| {
                    m.content("The user you are trying to unmute is not muted")
                })
                .await;
            if let Err(e) = message {
                println!("Error sending message {}", e);
            }
        }
    }
    Ok(())
}
