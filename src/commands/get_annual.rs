use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::model::application::{CommandOptionType, ResolvedOption, ResolvedValue};

pub fn run(options: &[ResolvedOption]) -> String {
    if let Some(ResolvedOption {
                    value: ResolvedValue::User(user, _), ..
                }) = options.first()
    {
        format!("SCANNED ID: {}", user.id)
    } else {
        "올바른 유저가 아닙니다.".to_string()
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("연차확인").description("연차를 확인합니다.").add_option(
        CreateCommandOption::new(CommandOptionType::User, "사용자", "연차를 확인할 사용자"),
    )
}