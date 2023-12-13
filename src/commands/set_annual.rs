use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::model::application::{CommandOptionType, ResolvedOption, ResolvedValue};

pub fn run(options: &[ResolvedOption]) -> String {
    if let Some(ResolvedOption {
                    value: ResolvedValue::User(user, _), ..
                }) = options.first()
    {
        if let Some(ResolvedOption {
                        value: ResolvedValue::Integer(count), ..
                    }) = options.last()
        {
            format!("SCANNED ID: {}, SCANNED NUMBER: {}", user.id, count)
        } else {
            "설정할 연차의 수를 입력해주세요.".to_string()
        }
    } else {
        "올바른 유저가 아닙니다.".to_string()
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("연차설정").description("연차를 설정합니다.").add_option(
        CreateCommandOption::new(CommandOptionType::User, "사용자", "연차를 설정할 사용자")
            .required(true),
    ).add_option(
        CreateCommandOption::new(CommandOptionType::Integer, "연차", "설정할 연차의 수")
            .required(true),
    )
}