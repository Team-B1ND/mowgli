use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::model::application::{CommandOptionType, ResolvedOption, ResolvedValue};
use serenity::prelude::Context;

pub fn run(ctx: &Context, options: &[ResolvedOption]) -> String {
    if let Some(ResolvedOption {
                    value: ResolvedValue::Integer(count), ..
                }) = options.first()
    {
        format!("SCANNED NUMBER: {}", count)
    } else {
        "1과 2 사이의 숫자를 입력해주세요.".to_string()
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("연차사용")
        .description("연차를 사용합니다.")
        .add_option(
            CreateCommandOption::new(CommandOptionType::Integer, "연차", "사용할 연차의 수")
                .required(true)
                .min_int_value(1)
                .max_int_value(2),
        )
}