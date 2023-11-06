use poise::serenity_prelude::{VoiceState, GuildId, UserId};
use serenity::{async_trait, prelude::{Context, EventHandler}};

pub struct Handler;

const GUILD_ID: GuildId = poise::serenity_prelude::GuildId(718661117483810816);
const TARGET_ID: UserId = poise::serenity_prelude::UserId(398298273699594240);
const ADDITIONAL_IDS: [UserId; 1] = [
    UserId(445421191302217739) ;
];

#[async_trait]
impl EventHandler for Handler {

    async fn voice_state_update(&self, ctx: Context, old_opt: Option<VoiceState>, new: VoiceState) {
 
        if new.guild_id.unwrap_or(poise::serenity_prelude::GuildId(0)) != GUILD_ID { return; }

        if new.user_id != TARGET_ID { return; }

        match old_opt {
            Some(old) => {
                if old.channel_id.is_some() { return; }
            },
            None => {},
        }

        let manager = songbird::get(&ctx).await.expect("Could not get songbird context :skull_emoji:");
        let song = songbird::ffmpeg("./foghorn.mp3").await.expect("Could not play foghorn.mp3");
        let handler = manager.join(new.guild_id.expect("This should exist by this point"), new.channel_id.expect("This should exist by this point")).await;

        let mut lock = handler.0.lock().await;

        let mut track = songbird::tracks::create_player(song);

        track.0.set_volume(0.2);

        lock.stop();

        lock.play(track.0);

        println!("Voice state updated FOG HORN ACTIVATED!!!!");


    }

}
