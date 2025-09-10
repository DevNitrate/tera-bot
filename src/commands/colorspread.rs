use scraper::{Html, Selector};
use serenity::all::{CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption, CreateEmbed, CreateInteractionResponseFollowup};

pub fn register() -> CreateCommand {
    let command_option = CreateCommandOption::new(CommandOptionType::Integer, "chapter_number", "number of the chapter to get colorspread from").required(true);

    CreateCommand::new("colorspread").add_option(command_option).description("gets the colorspread of the corresponding chapter")
}

pub async fn run(ctx: &Context, interaction: &CommandInteraction) {
    let _ = interaction.defer(&ctx.http).await;
    let opt = interaction.data.options.iter().next().unwrap();
    let chapter_num = opt.value.as_i64().unwrap();

    let response: CreateInteractionResponseFollowup;

    let base_url: &str = "https://onepiece.fandom.com/wiki/Category:Color_Spreads";
    let res = reqwest::get(base_url).await;

    let mut img_vec: Vec<String> = Vec::new();

    if let Ok(r) = res {
        let html_content = r.text().await.unwrap();

        let document = Html::parse_document(&html_content);
        let selector = Selector::parse("li.category-page__member").unwrap();

        let lis = document.select(&selector);

        for li in lis {
            let img_url: &str = li.select(&Selector::parse("img").unwrap()).next().unwrap().value().attr("src").unwrap();

            if img_url.contains(format!("Chapter_{}", chapter_num).as_str()) {
                let colorspread_url = img_url.split_once("/revision").unwrap().0;
                println!("{}", colorspread_url);
                img_vec.push(colorspread_url.to_string());
            }
        }

        if !img_vec.is_empty() {
            response = CreateInteractionResponseFollowup::new().embed(CreateEmbed::new().image(img_vec[0].clone()).description(format!("## colorspread chapter {}", chapter_num)));
        } else {
            response = CreateInteractionResponseFollowup::new().content(format!("***Chapter number {} does not feature a colorspread***", chapter_num));
        }
    } else {
        response = CreateInteractionResponseFollowup::new().content(format!("***Error getting base link***"));
    }


    let msg = interaction.create_followup(&ctx.http, response).await;

    if let Err(e) = msg {
        println!("***failed to create colorspread response: {}***", e);
    }

    // ""
}