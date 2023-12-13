use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

pub fn run(_options: &[ResolvedOption]) -> String {
    "반가워!".to_string()
}

pub fn register() -> CreateCommand {
    CreateCommand::new("연차사용").description("연차를 사용합니다.")
}