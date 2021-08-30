/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use twilight_embed_builder::{EmbedBuilder, EmbedFieldBuilder, EmbedFooterBuilder, ImageSource};

pub async fn ban(
    http: twilight_http::Client,
    msg: Box<twilight_model::gateway::payload::MessageCreate>,
) -> Result<&'static str, Box<dyn std::error::Error + Send + Sync>> {
    if !msg.author.bot {
        http.create_ban(msg.guild_id.unwrap(), msg.mentions[0].id)
            .exec()
            .await?;

        let famfo_avatar = ImageSource::url("https://avatars.githubusercontent.com/u/44938471")?;
        let embed = EmbedBuilder::new()
            .color(0x16990a)
            .title("User banned.")
            .description("User sucessfully banned.")
            .footer(EmbedFooterBuilder::new("Bot coded in rust by famfo.").icon_url(famfo_avatar))
            .build()?;

        http.create_message(msg.channel_id)
            .embeds(&[embed])?
            .exec()
            .await?;

        return Ok("Banned member.");
    }
    Ok("Message was send by a bot.")
}

pub async fn unban(
    http: twilight_http::Client,
    msg: Box<twilight_model::gateway::payload::MessageCreate>,
) -> Result<&'static str, Box<dyn std::error::Error + Send + Sync>> {
    if !msg.author.bot {
        let user_id0: Vec<&str> = msg.content.split_whitespace().collect();
        let famfo_avatar = ImageSource::url("https://avatars.githubusercontent.com/u/44938471")?;

        if user_id0.capacity() >= 2 {
            let user_id1 = user_id0[1].parse::<u64>();
            if let Err(_e) = user_id1 {
                let embed = EmbedBuilder::new()
                    .color(0x16990a)
                    .description("Failed to get an appropriate UserId.")
                    .footer(
                        EmbedFooterBuilder::new("Bot coded in rust by famfo.")
                            .icon_url(famfo_avatar),
                    )
                    .build()?;

                http.create_message(msg.channel_id)
                    .embeds(&[embed])?
                    .exec()
                    .await?;

                return Ok("Failed to get appropriate UserID.");
            } else {
                http.delete_ban(
                    msg.guild_id.unwrap(),
                    twilight_model::id::UserId(user_id1.unwrap()),
                )
                .exec()
                .await?;

                let embed = EmbedBuilder::new()
                    .color(0x16990a)
                    .title("User unbanned.")
                    .description("User sucessfully unbanned.")
                    .footer(
                        EmbedFooterBuilder::new("Bot coded in rust by famfo.")
                            .icon_url(famfo_avatar),
                    )
                    .build()?;

                http.create_message(msg.channel_id)
                    .embeds(&[embed])?
                    .exec()
                    .await?;

                return Ok("User unbanned.");
            }
        } else {
            let embed = EmbedBuilder::new()
                .color(0x16990a)
                .description("Failed to get an appropriate UserId.")
                .footer(
                    EmbedFooterBuilder::new("Bot coded in rust by famfo.").icon_url(famfo_avatar),
                )
                .build()?;

            http.create_message(msg.channel_id)
                .embeds(&[embed])?
                .exec()
                .await?;

            return Ok("Failed to get appropriate UserID.");
        }
    }
    Ok("Message was send by a bot.")
}

pub async fn kick(
    http: twilight_http::Client,
    msg: Box<twilight_model::gateway::payload::MessageCreate>,
) -> Result<&'static str, Box<dyn std::error::Error + Send + Sync>> {
    if !msg.author.bot {
        http.remove_guild_member(msg.guild_id.unwrap(), msg.mentions[0].id)
            .exec()
            .await?;

        let famfo_avatar = ImageSource::url("https://avatars.githubusercontent.com/u/44938471")?;
        let embed = EmbedBuilder::new()
            .color(0x16990a)
            .title("User kicked.")
            .description("User sucessfully kicked.")
            .footer(EmbedFooterBuilder::new("Bot coded in rust by famfo.").icon_url(famfo_avatar))
            .build()?;

        http.create_message(msg.channel_id)
            .embeds(&[embed])?
            .exec()
            .await?;

        return Ok("Member kicked.");
    }
    Ok("Message was send by a bot.")
}

pub async fn help(
    http: twilight_http::Client,
    msg: Box<twilight_model::gateway::payload::MessageCreate>,
) -> Result<&'static str, Box<dyn std::error::Error + Send + Sync>> {
    if !msg.author.bot {
        let famfo_avatar = ImageSource::url("https://avatars.githubusercontent.com/u/44938471")?;
        let yamb_avatar = ImageSource::url("https://cdn.discordapp.com/attachments/866417447837761546/881928893228023828/yamb1.png")?;
        let embed = EmbedBuilder::new()
            .color(0x16990a)
            .title("Yet Another Moderation Bot")
            .url("https://github.com/famfo/discord_bot-rs")
            .thumbnail(yamb_avatar)
            .description("Currently available bot commands:")
            .field(EmbedFieldBuilder::new(
                "$help",
                "Shows all available commands.",
            ))
            .field(EmbedFieldBuilder::new("$ban", "Bans the mentioned user."))
            .field(EmbedFieldBuilder::new(
                "$unban",
                "Unbans a member with their given UserId.",
            ))
            .field(EmbedFieldBuilder::new(
                "$kick",
                "Kicks the mentioned member.",
            ))
            .footer(EmbedFooterBuilder::new("Bot coded in rust by famfo.").icon_url(famfo_avatar))
            .build()?;

        http.create_message(msg.channel_id)
            .embeds(&[embed])?
            .exec()
            .await?;

        return Ok("Help send.");
    }
    Ok("Message was send by a bot.")
}
