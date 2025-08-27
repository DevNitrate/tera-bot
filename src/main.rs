use std::{fs, thread::sleep, time::Duration};

use serenity::{all::{ChannelId, ClientBuilder, Context, CreateMessage, EventHandler, GatewayIntents, GuildId, Interaction, Ready}, async_trait, Client};

use crate::commands::Chapter;

struct Handler;

mod commands;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _ready: Ready) {
        println!("bot running");

        let guild_id = GuildId::new(dotenv::var("GUILD_ID").unwrap().parse::<u64>().unwrap());

        let _commands = guild_id.set_commands(&ctx.http, vec![
            commands::ping::register(),
            commands::getchapter::register()
        ]).await;

        tokio::spawn(async move {
            let channel_id: ChannelId = ChannelId::new(dotenv::var("CHANNEL_ID").unwrap().parse::<u64>().unwrap());
            let tmp_file: &str = "/tmp/tera-latest";
            
            loop {
                let contents = fs::read_to_string(tmp_file).unwrap();
                let latest = Chapter::get(1157).await.unwrap();

                if contents.trim().parse::<i64>().unwrap() < latest.number {
                    fs::write(tmp_file, latest.number.to_string()).unwrap();

                    let create_msg = CreateMessage::new().content(format!("# NEW CHAPTER IS OUT! @everyone\n{}", latest.to_medium_header()));

                    let msg = channel_id.send_message(&ctx.http, create_msg).await;

                    if let Err(e) = msg {
                        println!("Error sending out message: {}", e);
                    }
                }

                sleep(Duration::from_secs(dotenv::var("SLEEP_DURATION").unwrap().parse::<u64>().unwrap()));
            }
        });
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(cmd) = interaction {
            match cmd.data.name.as_str() {
                "ping" => commands::ping::run(&ctx, &cmd).await,
                "getchapter" => commands::getchapter::run(&ctx, &cmd).await,
                _ => {}
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let token: String = dotenv::var("TOKEN").unwrap();
    let intents: GatewayIntents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client: Client = ClientBuilder::new(&token, intents).event_handler(Handler).await.expect("failed to create client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }

    println!("{}", token);
}
