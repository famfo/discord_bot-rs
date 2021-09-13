use crate::twilight_embed;

use twilight_embed_builder::{EmbedBuilder, EmbedFooterBuilder, ImageSource};

pub async fn ban(
    http: twilight_http::Client,
    msg: Box<twilight_model::gateway::payload::MessageCreate>,
) -> Result<&'static str, Box<dyn std::error::Error + Send + Sync>> {
    if !msg.author.bot {
        if msg.mentions.len() == 0 {
            let embed = twilight_embed!(" ", "No user mentioned.");
            http.create_message(msg.channel_id)
                .embeds(&[embed])?
                .exec()
                .await?;

            return Ok("No user mentioned");
        } else {
            http.create_ban(msg.guild_id.unwrap(), msg.mentions[0].id)
                .exec()
                .await?;

            let embed = twilight_embed!("User banned.", "User sucessfully banned.");
            http.create_message(msg.channel_id)
                .embeds(&[embed])?
                .exec()
                .await?;

            return Ok("Banned member.");
        }
    }
    Ok("Message was send by a bot.")
}
