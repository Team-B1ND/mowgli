use serenity::builder::{CreateCommand, CreateCommandOption, CreateEmbed};
use serenity::model::Colour;
use serenity::model::application::{CommandOptionType, ResolvedOption};
use serenity::prelude::Context;

pub async fn run(ctx: &Context,
                 options: &[ResolvedOption<'_>]) -> CreateEmbed {
    if let Some(ResolvedOption {
                    value: ResolvedValue::Integer(count), ..
                }) = options.first()
    {
        format!("SCANNED NUMBER: {}", count)
    } else {
        "1과 2 사이의 숫자를 입력해주세요.".to_string()
    }
    CreateEmbed::new()
        .title("This is a title")
        .description("This is a description")
        .color(Colour::new(0x5E311F))
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