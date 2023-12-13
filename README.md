![Mowgli](https://raw.githubusercontent.com/Team-B1ND/mowgli/main/logo.png)
## 소개
바인드팀 디스코드를 책임질 귀여운 모글리
## 사용법
1. 최상위 경로 `.env` 파일에 다음 값 기입
- `TOKEN`: 디스코드 봇의 토큰
- `DATABASE_URL`: 데이터베이스의 경로(상대경로 사용 불가)
- `ADMIN`: 관리자의 디스코드 ID(숫자 Snowflake)
```dotenv
TOKEN=D0HYeoNw00KsN0sE.1ST0oB1GT0HAvE.AG1RLFR1eND
DATABASE_URL=/tmp/mowgli.sqlite
ADMIN=4158255151423151511
```
2. 최상위 경로에서 다음 명령 실행
- `Cargo` 및 `Rust` 필요
```shell
cargo install diesel_cli
diesel migration run
```
## 스택
`Rust`, `Serenity`, `Diesel-SQLite3`