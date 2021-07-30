/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

extern crate systemstat;

use serenity::client::{Client, Context, EventHandler};
use serenity::framework::standard::{
    macros::{command, group},
    Args, CommandResult, StandardFramework,
};
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::{async_trait, model::id::UserId};

use systemstat::{saturating_sub_bytes, Platform, System};

use dotenv;
use std::env;
use std::fs;
use std::thread;
use std::time::Duration;

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
    unmute,
    join_vc,
    leave_vc,
    play
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

#[command]
#[only_in(guilds)]
async fn ban(ctx: &Context, msg: &Message) -> CommandResult {
    // check if the member has rights to ban users
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
        if msg.mentions.is_empty() {
            let message = msg
                .channel_id
                .send_message(&ctx, |m| m.content("No user to ban provided."))
                .await;
            if let Err(e) = message {
                println!("Error sending message: {}", e);
            }

            return Ok(());
        }

        // member to ban
        let member = &msg.mentions[0];
        // guild the message is sent in
        let guild = msg.guild_id.unwrap();

        // ban the member from the guild
        if let Ok(()) = guild.ban(&ctx, member, 0).await {
            println!("Successfully banned member");
        }
        let message = msg.channel_id.say(&ctx, "User banned").await;
        if let Err(e) = message {
            println!("Error sending message: {}", e);
        }
    } else {
        let message = msg
            .channel_id
            .say(&ctx, "You dont have permissions to ban members")
            .await;
        if let Err(e) = message {
            println!("Error sending message: {}", e);
        }
    }
    Ok(())
}

#[command]
#[only_in(guilds)]
async fn kick(ctx: &Context, msg: &Message) -> CommandResult {
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
        if msg.mentions.is_empty() {
            let message = msg
                .channel_id
                .send_message(&ctx, |m| m.content("No uset to kick provided."))
                .await;
            if let Err(e) = message {
                println!("Error sending message: {}", e);
            }

            return Ok(());
        }

        let member = &msg.mentions[0];
        let guild = msg.guild_id.unwrap();

        if let Ok(()) = guild.kick(ctx, member).await {
            println!("Successfully kicked member");
        }
        let message = msg.channel_id.say(&ctx, "User kicked").await;
        if let Err(e) = message {
            println!("Error sending message: {}", e);
        }
    } else {
        let message = msg
            .channel_id
            .say(&ctx, "You dont have permissions to kick members")
            .await;
        if let Err(e) = message {
            println!("Error sending message: {}", e);
        }
    }
    Ok(())
}

#[command]
#[only_in(guilds)]
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
        if msg.mentions.is_empty() {
            let message = msg
                .channel_id
                .send_message(&ctx, |m| m.content("No user to mute provided."))
                .await;
            if let Err(e) = message {
                println!("Error sending message: {}", e);
            }

            return Ok(());
        }

        let user_id = &msg.mentions[0];

        // Get the guild the member is in
        if let Some(guild) = msg.guild_id.unwrap().to_guild_cached(&ctx).await {
            // Get the muted role by its name
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

                let message = msg.channel_id.say(&ctx, "User muted").await;
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

#[command]
#[only_in(guilds)]
async fn unban(ctx: &Context, msg: &Message) -> CommandResult {
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
        if msg.mentions.is_empty() {
            let message = msg
                .channel_id
                .send_message(&ctx, |m| m.content("No user to unban provided."))
                .await;
            if let Err(e) = message {
                println!("Error sending message: {}", e);
            }

            return Ok(());
        }

        let user_id = msg.content[2..].split_once(" ").unwrap();
        let user_id = UserId(user_id.1.parse::<u64>().unwrap());
        let guild = msg.guild_id.unwrap();

        if let Ok(()) = guild.unban(&ctx, user_id).await {
            println!("Successfully unbanned member");
        }
        let message = msg.channel_id.say(&ctx, "User unbanned").await;
        if let Err(e) = message {
            println!("Error sending message: {}", e);
        }
    } else {
        let message = msg
            .channel_id
            .say(&ctx, "You dont have permissions to unban members")
            .await;
        if let Err(e) = message {
            println!("Error sending message: {}", e);
        }
    }
    Ok(())
}

#[command]
#[only_in(guilds)]
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
        if msg.mentions.is_empty() {
            let message = msg
                .channel_id
                .send_message(&ctx, |m| m.content("No user to unmute provided."))
                .await;
            if let Err(e) = message {
                println!("Error sending message: {}", e);
            }

            return Ok(());
        }

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
                // Get the muted role by its name
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

                    let message = msg.channel_id.say(&ctx, "User unmuted").await;
                    if let Err(e) = message {
                        println!("Error sending message {}", e);
                    }
                }
            }
        } else {
            let message = msg
                .channel_id
                .say(ctx, "The user you are trying to unmute is not muted")
                .await;
            if let Err(e) = message {
                println!("Error sending message {}", e);
            }
        }
    }
    Ok(())
}

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
                .field("$leave_vc", "Leaves the Voice Channel the user is in.", false)
                .field("$mute", "Mutes a member.", false)
                .field("$play", "Plays audio from a given URL", false)
                .field("$poll", "Starts a poll.", false)
                .field("$the_missile", "The missile knows where it is...", false)
                .field("$unban", "Unbans a user by their user ID.", false)
                .field("$unmute", "Unmutes a member.", false)
                .field("$join_vc", "Joins the Voice Channel the user is in.", false)
                .field("License", "This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.", false)
                .field("Source Code", "https://github.com/famfo/discord_bot-rs", false)
                .field("Issues", "If you have questions, encounter bugs or have feature requests, considering opening an issue on github", false)
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

#[command]
async fn bot_info(ctx: &Context, msg: &Message) -> CommandResult {
    let sys = System::new();
    let message = msg
        .channel_id
        .say(&ctx, "Collecting system information...")
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

#[command]
async fn foo(ctx: &Context, msg: &Message) -> CommandResult {
    // Get the content of the send message
    let args = msg.content[4..].split_once(" ").unwrap();
    let to_send = format!("{} | {}", args.1, msg.author.name);
    let message = msg.channel_id.say(&ctx, to_send).await;
    if let Err(e) = message {
        println!("Error sending message: {}", e);
    }
    Ok(())
}

#[command]
async fn duden(ctx: &Context, msg: &Message) -> CommandResult {
    let duden =
        fs::read_to_string("external/duden").expect("Something went wrong reading the file");
    let message = msg.channel_id.say(&ctx, &duden).await;
    if let Err(e) = message {
        println!("Error sending message: {}", e);
    }
    Ok(())
}

#[command]
async fn poll(ctx: &Context, msg: &Message) -> CommandResult {
    if msg.content == "$poll" {
        let empty_message = msg.channel_id.say(ctx, "No topic for the poll.").await;

        if let Err(e) = empty_message {
            println!("Error sending message: {}", e);
        }
    } else {
        let args = msg.content[2..].split_once(" ").unwrap();
        let topic = format!("Poll by {}:", msg.author.name);
        let embed = msg
            .channel_id
            .send_message(&ctx, |m| {
                m.embed(|e| {
                    e.title(topic).description(args.1).footer(|f| {
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

#[command]
#[only_in(guilds)]
async fn join_vc(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guild_id = guild.id;

    let channel_id = guild
        .voice_states
        .get(&msg.author.id)
        .and_then(|voice_state| voice_state.channel_id);

    let connect_to = if let Some(channel) = channel_id {
        channel
    } else {
        let message = msg.channel_id.say(ctx, "Not in a voice channel").await;
        if let Err(e) = message {
            println!("Error sending message {}", e);
        }

        return Ok(());
    };

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    let _handler = manager.join(guild_id, connect_to).await;

    Ok(())
}

#[command]
#[only_in(guilds)]
async fn play(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let url = match args.single::<String>() {
        Ok(url) => url,
        Err(_) => {
            let message = msg
                .channel_id
                .say(&ctx.http, "Must provide a URL to a video or audio")
                .await;
            if let Err(e) = message {
                println!("Error sending message {}", e);
            }

            return Ok(());
        }
    };

    if !url.starts_with("http") {
        let message = msg
            .channel_id
            .say(&ctx.http, "Must provide a valid URL")
            .await;
        if let Err(e) = message {
            println!("Error sending message {}", e);
        }

        return Ok(());
    }

    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        let source = match songbird::ytdl(&url).await {
            Ok(source) => source,
            Err(e) => {
                println!("Err starting source: {:?}", e);

                let message = msg.channel_id.say(&ctx.http, "Error sourcing ffmpeg").await;
                if let Err(e) = message {
                    println!("Error sending message {}", e);
                }

                return Ok(());
            }
        };

        handler.play_source(source);

        let message = msg.channel_id.say(&ctx.http, "Playing song").await;
        if let Err(e) = message {
            println!("Error sending message {}", e);
        }
    } else {
        let message = msg
            .channel_id
            .say(&ctx.http, "Not in a voice channel to play in")
            .await;
        if let Err(e) = message {
            println!("Error sending message {}", e);
        }
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
async fn leave_vc(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild_id.unwrap();

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();
    let has_handler = manager.get(guild).is_some();

    if has_handler {
        if let Err(e) = manager.remove(guild).await {
            let message = msg
                .channel_id
                .say(&ctx.http, format!("Failed: {:?}", e))
                .await;
            if let Err(e) = message {
                println!("Error sending message {}", e);
            }
        }

        let message = msg.channel_id.say(&ctx.http, "Left voice channel").await;
        if let Err(e) = message {
            println!("Error sending message {}", e);
        }
    } else {
        let message = msg
            .channel_id
            .say(&ctx.http, "Bot is not in a voice channel")
            .await;
        if let Err(e) = message {
            println!("Error sending message {}", e);
        }
    }
    Ok(())
}
