use serenity::all::{CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption, CreateEmbed, CreateInteractionResponseFollowup};

pub fn register() -> CreateCommand {
    let command_option = CreateCommandOption::new(CommandOptionType::User, "getpfp", "gets the pfp of specified user").required(true);

    CreateCommand::new("getpfp").add_option(command_option).description("get the pfp of specified user")
}

pub async fn run(ctx: &Context, interaction: &CommandInteraction) {
    let _ = interaction.defer(&ctx.http).await;
    let opt = interaction.data.options.iter().next().unwrap();
    let user_id = opt.value.as_user_id().unwrap();

    let response: CreateInteractionResponseFollowup;
    
    if let Some(user) = interaction.data.resolved.users.get(&user_id) {
        let avatar_url_res = user.avatar_url();
        let avatar_url_final: String;

        if let Some(mut avatar_url) = avatar_url_res {
            avatar_url.truncate(avatar_url.len()-4);
            avatar_url.push_str("2048");

            avatar_url_final = avatar_url;
        } else {
            let avatar_url = user.default_avatar_url();
            avatar_url_final = avatar_url;
        }
        response = CreateInteractionResponseFollowup::new().embed(CreateEmbed::new().image(avatar_url_final).description(format!("## pfp of following user: <@{}>", user.id)));
    } else {
        response = CreateInteractionResponseFollowup::new().content("***could not fetch user***");
    }

    let msg = interaction.create_followup(&ctx.http, response).await;

    if let Err(e) = msg {
        println!("***failed to create getpfp response: {}***", e);
    }
}