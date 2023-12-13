use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect(".env 파일에 URL 없음");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("{} 연결 오류!", database_url))
}