mod commands;

use std::env;
use dotenv::dotenv;

use serenity::async_trait;
use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseMessage};
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
            commands::annual::register(),
        ])
            .await
            .expect("명령 생성 실패");
    }
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            let content = match command.data.name.as_str() {
                "안녕" => Some(commands::annual::run(&command.data.options())),
                _ => Some("없는 명령어입니다 :(".to_string()),
            };

            if let Some(content) = content {
                let data = CreateInteractionResponseMessage::new().content(content);
                let builder = CreateInteractionResponse::Message(data);
                if let Err(why) = command.create_response(&ctx.http, builder).await {
                    println!("Cannot respond to slash command: {why}");
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("TOKEN").expect(".env 파일에 토큰 없음");

    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .await
        .expect("클라이언트 생성 실패");

    if let Err(why) = client.start().await {
        println!("클라이언트 오류: {:?}", why);
    }
}