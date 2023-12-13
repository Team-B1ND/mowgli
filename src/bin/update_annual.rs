use crate::schema::users::dsl::users;
use crate::schema::users::annual;
use diesel::prelude::*;
use mowgli::establish_connection;

pub fn update_annual(id: i64, value: i32) {
    let connection = &mut establish_connection();
    diesel::update(users.find(id))
        .set(annual.eq(value))
        .execute(connection)
        .expect("사용자 수정에 실패했습니다.");
}