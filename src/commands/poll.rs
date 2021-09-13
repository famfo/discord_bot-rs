/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use crate::twilight_embed;

use twilight_embed_builder::{EmbedBuilder, EmbedFooterBuilder, ImageSource};

pub async fn poll(
    http: twilight_http::Client,
    msg: Box<twilight_model::gateway::payload::MessageCreate>,
) -> Result<&'static str, Box<dyn std::error::Error + Send + Sync>> {
    if !msg.author.bot {
        let content = msg.content.split_at(5);

        if content.1 != "" {
            let title = format!("Poll by {}", msg.author.name);
            let embed = twilight_embed!(title, content.1);
            let emoji_yes =
                twilight_http::request::channel::reaction::RequestReactionType::Unicode {
                    name: "✅",
                };
            let emoji_no =
                twilight_http::request::channel::reaction::RequestReactionType::Unicode {
                    name: "❌",
                };

            let reaction_message_id = http
                .create_message(msg.channel_id)
                .embeds(&[embed])?
                .exec()
                .await?
                .model()
                .await?
                .id;

            http.create_reaction(msg.channel_id, reaction_message_id, &emoji_yes)
                .exec()
                .await?;

            http.create_reaction(msg.channel_id, reaction_message_id, &emoji_no)
                .exec()
                .await?;

            return Ok("Embed send.");
        } else {
            let embed = twilight_embed!(" ", "No topic for poll.");

            http.create_message(msg.channel_id)
                .embeds(&[embed])?
                .exec()
                .await?;

            return Ok("No topic for poll.");
        }
    }
    Ok("Message was send by a bot.")
}
