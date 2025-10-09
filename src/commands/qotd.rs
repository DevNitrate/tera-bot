use std::{env::home_dir, fs, path::Path};

use serenity::all::{CommandInteraction, Context, CreateCommand, CreateInteractionResponseFollowup};

pub fn register() -> CreateCommand {
    CreateCommand::new("qotd").description("Gives the question of the day")
}

pub async fn run(ctx: &Context, interaction: &CommandInteraction) {
    let _ = interaction.defer(&ctx.http).await;

    let response: CreateInteractionResponseFollowup;

    let home_dir = home_dir().unwrap();
    let question_file_path = format!("{}/.tera-bot/qotd", home_dir.to_str().unwrap());
    let user_file_path = format!("{}/.tera-bot/qotd_user", home_dir.to_str().unwrap());

    if !Path::new(&question_file_path).exists() {
        response = CreateInteractionResponseFollowup::new().content(format!("## no question is set please use the ``/setquestion`` command"));
    } else {
        let question = fs::read_to_string(question_file_path).unwrap().trim().to_string();
        let user = fs::read_to_string(user_file_path).unwrap().trim().to_string();

        response = CreateInteractionResponseFollowup::new().content(format!("# question of the day from <@{}>:\n# {}", user, question));   
    }

    let msg = interaction.create_followup(&ctx.http, response).await;

    if let Err(e) = msg {
        println!("***failed to create setquestion response: {}***", e);
    }
}