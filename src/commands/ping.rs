use serenity::all::{CommandInteraction, Context, CreateCommand, CreateInteractionResponse, CreateInteractionResponseMessage};

pub fn register() -> CreateCommand {
    CreateCommand::new("ping").description("ping the bot")
}

pub async fn run(ctx: &Context, interaction: &CommandInteraction) {
    let response = CreateInteractionResponseMessage::new().content("pong");
    let response = CreateInteractionResponse::Message(response);

    let msg = interaction.create_response(&ctx.http, response).await;

    if let Err(e) = msg {
        println!("failed to create ping response: {}", e);
    }
}