use serenity::builder::{CreateCommand, CreateCommandOption, CreateEmbed};
use serenity::model::application::{CommandOptionType, ResolvedOption, ResolvedValue};
use serenity::model::Colour;
use serenity::prelude::Context;
use crate::bin::create_annual::create_annual;
use crate::bin::read_annual::read_annual;
use crate::bin::update_annual::update_annual;

pub async fn run(ctx: &Context,
                 options: &[ResolvedOption<'_>]) -> CreateEmbed {
    if let Some(ResolvedOption {
                    value: ResolvedValue::Integer(count), ..
                }) = options.first() {
        let user = ctx.http.get_current_user().await.unwrap();
        let id = i64::from(user.id);
        create_annual(id);
        let name = &user.name;
        let annual = read_annual(id).unwrap_or(0);
        let value = annual-*count as i32;
        update_annual(id, value);
        CreateEmbed::new()
            .title(format!("연차 {count}개를 사용하셨습니다."))
            .description(format!("{name}님의 남은 연차는 이제 {value}개 입니다."))
            .color(Colour::new(0x5E311F))
    } else {
        CreateEmbed::new()
            .title("1과 2 사이의 숫자를 입력해주세요!")
            .color(Colour::new(0xFF0000))
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