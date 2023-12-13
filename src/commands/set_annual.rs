use serenity::builder::{CreateCommand, CreateCommandOption, CreateEmbed};
use serenity::model::application::{CommandOptionType, ResolvedOption, ResolvedValue};
use serenity::model::Colour;
use serenity::prelude::Context;
use crate::bin::create_annual::create_annual;
use crate::bin::update_annual::update_annual;

pub async fn run(ctx: &Context,
                 options: &[ResolvedOption<'_>]) -> CreateEmbed {
    if let Some(ResolvedOption {
                    value: ResolvedValue::User(user, _), ..
                }) = options.first() {
        if let Some(ResolvedOption {
                        value: ResolvedValue::Integer(count), ..
                    }) = options.last() {

            let id = i64::from(user.id);
            create_annual(id);
            let name = &user.name;
            update_annual(id, *count as i32);
            CreateEmbed::new()
                .title(format!("{name}님의 연차는 이제 {count}개 입니다."))
                .color(Colour::new(0x5E311F))
        } else {
            CreateEmbed::new()
                .title("설정할 연차의 수를 입력해주세요!")
                .color(Colour::new(0xFF0000))
        }
    } else {
        CreateEmbed::new()
            .title("올바른 유저가 아닙니다!")
            .color(Colour::new(0xFF0000))
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("연차설정")
        .description("연차를 설정합니다.")
        .add_option(
            CreateCommandOption::new(CommandOptionType::User, "사용자", "연차를 설정할 사용자")
                .required(true),
        )
        .add_option(
            CreateCommandOption::new(CommandOptionType::Integer, "연차", "설정할 연차의 수")
                .required(true),
        )
}