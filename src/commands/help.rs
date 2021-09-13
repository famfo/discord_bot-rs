use twilight_embed_builder::{EmbedBuilder, EmbedFieldBuilder, EmbedFooterBuilder, ImageSource};

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
