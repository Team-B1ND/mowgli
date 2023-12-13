use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

pub fn run(_options: &[ResolvedOption]) -> String {
    "반가워!".to_string()
}

pub fn register() -> CreateCommand {
    CreateCommand::new("안녕").description("모글리가 인사합니다.")
}