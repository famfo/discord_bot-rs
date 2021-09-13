/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use crate::twilight_embed;

use twilight_embed_builder::{EmbedBuilder, EmbedFooterBuilder, ImageSource};

pub async fn unban(
    http: twilight_http::Client,
    msg: Box<twilight_model::gateway::payload::MessageCreate>,
) -> Result<&'static str, Box<dyn std::error::Error + Send + Sync>> {
    if !msg.author.bot {
        let user_id0: Vec<&str> = msg.content.split_whitespace().collect();

        if user_id0.capacity() >= 2 {
            let user_id1 = user_id0[1].parse::<u64>();
            if let Err(_e) = user_id1 {
                let embed = twilight_embed!(" ", "Failed to get an appropriate UserId.");
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

                let embed = twilight_embed!("User unbanned.", "User sucessfully unbanned.");
                http.create_message(msg.channel_id)
                    .embeds(&[embed])?
                    .exec()
                    .await?;

                return Ok("User unbanned.");
            }
        } else {
            let embed = twilight_embed!(" ", "Failed to get an appropriate UserId.");
            http.create_message(msg.channel_id)
                .embeds(&[embed])?
                .exec()
                .await?;

            return Ok("Failed to get appropriate UserID.");
        }
    }
    Ok("Message was send by a bot.")
}
