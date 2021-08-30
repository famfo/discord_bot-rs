/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use futures::stream::StreamExt;
use std::{env, error::Error};
use twilight_cache_inmemory::{InMemoryCache, ResourceType};
use twilight_gateway::{
    cluster::{Cluster, ShardScheme},
    Event,
};

mod commands;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    dotenv::dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the enviroment");
    let scheme = ShardScheme::Auto;
    let (cluster, mut events) = Cluster::builder(
        token.to_owned(),
        twilight_model::gateway::Intents::GUILD_MESSAGES,
    )
    .shard_scheme(scheme)
    .build()
    .await?;

    let cluster_spawn = cluster.clone();

    tokio::spawn(async move {
        cluster_spawn.up().await;
    });

    let http = twilight_http::Client::new(token);
    let cache = InMemoryCache::builder()
        .resource_types(ResourceType::MESSAGE)
        .build();

    while let Some((shard_id, event)) = events.next().await {
        cache.update(&event);
        tokio::spawn(handle_event(shard_id, event, http.clone()));
    }
    Ok(())
}

async fn handle_event(
    shard_id: u64,
    event: Event,
    http: twilight_http::Client,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match event {
        Event::MessageCreate(msg) if msg.content.to_lowercase().starts_with("$help") => {
            commands::help(http, msg).await?;
        }
        Event::MessageCreate(msg) if msg.content.to_lowercase().starts_with("$ban") => {
            commands::ban(http, msg).await?;
        }
        Event::MessageCreate(msg) if msg.content.to_lowercase().starts_with("$unban") => {
            commands::unban(http, msg).await?;
        }
        Event::MessageCreate(msg) if msg.content.to_lowercase().starts_with("$kick") => {
            commands::kick(http, msg).await?;
        }
        Event::ShardConnected(_) => {
            println!("Connected on shard {}", shard_id);
        }
        _ => {}
    }

    Ok(())
}
