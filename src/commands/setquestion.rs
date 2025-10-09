use std::{env::home_dir, fs::{File, OpenOptions}, io::Write, path::Path, time::{Duration, SystemTime}};

use serenity::all::{CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption, CreateInteractionResponseFollowup};

pub fn register() -> CreateCommand {
    let command_options = CreateCommandOption::new(CommandOptionType::String, "setquestion", "Sets the question of the day").required(true);

    CreateCommand::new("setquestion").add_option(command_options).description("Sets the question of the day")
}

pub async fn run(ctx: &Context, interaction: &CommandInteraction) {
    let _ = interaction.defer(&ctx.http).await;
    let opt = interaction.data.options.iter().next().unwrap();
    let question = opt.value.as_str().unwrap();

    let user = interaction.user.id;

    let home_dir = home_dir().unwrap();
    let question_file_path = format!("{}/.tera-bot/qotd", home_dir.to_str().unwrap());
    let user_file_path = format!("{}/.tera-bot/qotd_user", home_dir.to_str().unwrap());
    
    let response: CreateInteractionResponseFollowup;

    if !Path::new(&question_file_path).exists() {
        let mut file = File::create(question_file_path).unwrap();
        writeln!(file, "{}", question).unwrap();

        let mut user_file = File::create(user_file_path).unwrap();
        writeln!(user_file, "{}", user).unwrap();

        response = CreateInteractionResponseFollowup::new().content(format!("# @everyone new question of the day from <@{}>:\n# {}", user, question));
    } else {
        let file_met = OpenOptions::new()
            .write(true)// enable writing
            .create(false)   // ensure it won’t be created if missing
            .open(&question_file_path).unwrap();

        let metadata = file_met.metadata().unwrap().modified().unwrap();
        let now = SystemTime::now();
        let elapsed = now.duration_since(metadata).unwrap();
        let cooldown = Duration::from_secs(6*60*60);

        let mut file = File::create(question_file_path).unwrap();
        
        if elapsed >= cooldown {
            writeln!(file, "{}", question).unwrap();
            
            let mut user_file = File::create(user_file_path).unwrap();
            writeln!(user_file, "{}", user).unwrap();

            response = CreateInteractionResponseFollowup::new().content(format!("# @everyone new question of the day from <@{}>:\n# {}", user, question));
        } else {
            response = CreateInteractionResponseFollowup::new().content(format!("## question of the day is already set please wait {}", format_duration(cooldown - elapsed)));
        }
    }

    let msg = interaction.create_followup(&ctx.http, response).await;

    if let Err(e) = msg {
        println!("***failed to create setquestion response: {}***", e);
    }    
}

fn format_duration(dur: Duration) -> String {
    let secs = dur.as_secs();

    let minute = 60;
    let hour = minute * 60;

    if secs >= hour {
        let (hours, hours_left) = (secs / hour, secs % hour);
        let (minutes, minutes_left) = (hours_left / minute, hours_left % minute);
        let seconds = minutes_left;
        return format!("{}h {}min {}s", hours, minutes, seconds);
    } else if secs >= minute {
        let (minutes, minutes_left) = (secs / minute, secs % minute);
        let seconds = minutes_left;
        return format!("{}min {}s", minutes, seconds);
    } else {
        return format!("{}s", secs);
    }
}