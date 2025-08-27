use serenity::all::{CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption, CreateInteractionResponseFollowup};

use crate::commands::Chapter;

pub fn register() -> CreateCommand {
    let command_option = CreateCommandOption::new(CommandOptionType::Integer, "chapter_number", "number of the chapter you want to get. use \"latest\" to get the latest chapter").required(true);

    CreateCommand::new("getchapter").add_option(command_option).description("Get info on a chapter")
}

pub async fn run(ctx: &Context, interaction: &CommandInteraction) {
    let _ = interaction.defer(&ctx.http).await;
    let opt = interaction.data.options.iter().next().unwrap();

    let response: CreateInteractionResponseFollowup;
    
    if opt.value.as_i64().unwrap() == 0 {
        let chapter = Chapter::get_latest().await;
        
        if let Some(chap) = chapter {
            response = CreateInteractionResponseFollowup::new().content(chap.to_medium_header());
        } else {
            response = CreateInteractionResponseFollowup::new().content("**failed to get latest chapter**");
        }
    } else {
        let chapter_number = opt.value.as_i64().unwrap();
        let chapter = Chapter::get(chapter_number).await;

        if let Some(chap) = chapter {
            response = CreateInteractionResponseFollowup::new().content(chap.to_medium_header());
        } else {
            response = CreateInteractionResponseFollowup::new().content(format!("**failed to get chapter {}**", chapter_number));
        }
    }

    let msg = interaction.create_followup(&ctx.http, response).await;

    if let Err(e) = msg {
        println!("**failed to create getchapter response: {}**", e);
    }
}