use serenity::builder::{CreateCommand, CreateCommandOption, CreateEmbed};
use serenity::model::application::{CommandOptionType, ResolvedOption, ResolvedValue};
use serenity::model::Colour;
use serenity::prelude::Context;
use crate::bin::create_annual::create_annual;
use crate::bin::read_annual::read_annual;

pub async fn run(ctx: &Context,
                 options: &[ResolvedOption<'_>]) -> CreateEmbed {
    let id;
    let name;
    let user = ctx.http.get_current_user().await.unwrap();
    if let Some(ResolvedOption {
                    value: ResolvedValue::User(user, _), ..
                }) = options.first() {
        id = user.id;
        name = &user.name;
    } else {
        id = user.id;
        name = &user.name;
    }
    let id_int = i64::from(user.id);
    create_annual(id_int);
    let annual = read_annual(id_int).unwrap_or(0);
    CreateEmbed::new()
        .title(format!("연차가 {annual}개 남았습니다."))
        .description(format!("{name}님의 정보"))
        .color(Colour::new(0x5E311F))
}

pub fn register() -> CreateCommand {
    CreateCommand::new("연차확인")
        .description("연차를 확인합니다.")
        .add_option(
            CreateCommandOption::new(CommandOptionType::User, "사용자", "연차를 확인할 사용자"),
        )
}