![Mowgli](https://raw.githubusercontent.com/Team-B1ND/mowgli/main/logo.png)
## 소개
바인드팀 디스코드를 책임질 귀여운 모글리
## 사용법
1. 최상위 경로 `.env` 파일에 `TOKEN`과 `DATABASE_URL` 기입.
```dotenv
TOKEN=D0HYeoNw00KsN0sE.1ST0oB1GT0HAvE.AG1RLFR1eND
DATABASE_URL=/tmp/mowgli.sqlite
```
2. 최상위 경로에서 다음 명령 실행.
```shell
cargo install diesel_cli
diesel migration run
```
## 스택
`Rust`, `Serenity`, `Diesel-SQLite3`