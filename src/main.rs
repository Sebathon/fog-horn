use dotenv::dotenv;

mod voice;
mod commands;
use songbird::SerenityInit;
use std::env;

pub struct Data {
}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

use serenity::prelude::GatewayIntents;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let discord_token = env::var("DISCORD_TOKEN")
        .expect("Missing DISCORD_TOKEN environment variable (fix this up)");

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![commands::ping::ping()],
            ..Default::default()
        })
        .token(discord_token)
            .client_settings(|builder| builder.register_songbird().event_handler(voice::voice::Handler))
        .intents(GatewayIntents::non_privileged())
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                })
            })
        });


    println!("Client is starting wooo!");
    framework.run().await.unwrap();
}
