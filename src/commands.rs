use twilight_http::Client as HttpClient;

pub async fn ban(
    http: HttpClient,
    msg: Box<twilight_model::gateway::payload::MessageCreate>,
) -> Result<&'static str, Box<dyn std::error::Error + Send + Sync>> {
    if !msg.author.bot {
        http.create_ban(msg.guild_id.unwrap(), msg.mentions[0].id)
            .exec()
            .await?;

        return Ok("Banned member.");
    }
    Ok("Message was send by a bot.")
}

pub async fn unban(
    http: HttpClient,
    msg: Box<twilight_model::gateway::payload::MessageCreate>,
) -> Result<&'static str, Box<dyn std::error::Error + Send + Sync>> {
    if !msg.author.bot {
        let user_id0: Vec<&str> = msg.content.split_whitespace().collect();
        if user_id0.capacity() >= 2 {
            let user_id1 = user_id0[1].parse::<u64>();
            if let Err(_e) = user_id1 {
                http.create_message(msg.channel_id)
                    .content("Failed to get appropriate UserID.")?
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
                return Ok("User unbanned.");
            }
        } else {
            http.create_message(msg.channel_id)
                .content("Failed to get appropriate UserID.")?
                .exec()
                .await?;
            return Ok("Failed to get appropriate UserID.");
        }
    }
    Ok("Message was send by a bot.")
}

pub async fn kick(
    http: HttpClient,
    msg: Box<twilight_model::gateway::payload::MessageCreate>,
) -> Result<&'static str, Box<dyn std::error::Error + Send + Sync>> {
    if !msg.author.bot {
        println!("Kick: {:?}", msg.mentions[0].id);
        http.remove_guild_member(msg.guild_id.unwrap(), msg.mentions[0].id)
            .exec()
            .await?;
        return Ok("Member kicked.");
    }
    Ok("Message was send by a bot.")
}

pub async fn help(
    http: HttpClient,
    msg: Box<twilight_model::gateway::payload::MessageCreate>,
) -> Result<&'static str, Box<dyn std::error::Error + Send + Sync>> {
    if !msg.author.bot {
        http.create_message(msg.channel_id)
            .content("Something something rewrite in twilight-rs.")?
            .exec()
            .await?;
        return Ok("Help send.");
    }
    Ok("Message was send by a bot.")
}
