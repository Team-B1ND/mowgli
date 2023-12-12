use std::env;
use dotenv::dotenv;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use serenity::gateway::ActivityData;

struct Handler;

#[async_trait]
impl EventHandler for Handler {

    async fn message(&self, ctx: Context, msg: Message) {
        let cmd = msg.content;

        if cmd.starts_with("") {

        }
    }
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} 봇 실행 완료!", ready.user.name);
        ctx.set_activity(Some(ActivityData::playing("도현욱 구타")));
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