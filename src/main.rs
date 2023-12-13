mod bin;
mod commands;
mod models;
mod schema;

use std::env;
use dotenv::dotenv;

use serenity::async_trait;
use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseMessage, CreateEmbed};
use serenity::model::Colour;
use serenity::model::application::{Command, Interaction};
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use serenity::gateway::ActivityData;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} 봇 실행 완료!", ready.user.name);
        ctx.set_activity(Some(ActivityData::playing("도현욱 구타")));

        Command::set_global_commands(&ctx.http, vec![
            commands::get_annual::register(),
            commands::set_annual::register(),
            commands::use_annual::register(),
        ])
            .await
            .expect("명령 생성에 실패했습니다.");
    }
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            let content = match command.data.name.as_str() {
                "연차확인" => {
                    commands::get_annual::run(&ctx, &command.data.options())
                        .await
                }
                "연차설정" => {
                    commands::set_annual::run(&ctx, &command.data.options())
                        .await
                }
                "연차사용" => {
                    commands::use_annual::run(&ctx, &command.data.options())
                        .await
                }
                _ => {
                    CreateEmbed::new()
                        .title("없는 명령어입니다!")
                        .color(Colour::new(0xFF0000))
                }
            };

            let data = CreateInteractionResponseMessage::new()
                .add_embed(content);
            let builder = CreateInteractionResponse::Message(data);
            if let Err(why) = command.create_response(&ctx.http, builder).await {
                println!("응답할 수 없습니다: {why}");
            }
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("TOKEN").expect(".env 파일에 토큰이 없습니다.");

    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .await
        .expect("클라이언트 생성에 실패했습니다.");

    if let Err(why) = client.start().await {
        println!("클라이언트 오류가 발생했습니다: {why}");
    }
}