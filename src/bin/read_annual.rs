use crate::models::User;
use crate::schema::users::dsl::users;
use diesel::prelude::*;
use mowgli::establish_connection;

pub fn read_annual(id: i64) -> Option<i32> {
    let connection = &mut establish_connection();
    let user = users
        .find(id)
        .select(User::as_select())
        .first(connection)
        .optional();
    match user {
        Ok(Some(user)) => Option::from(user.annual),
        Ok(None) => {
            println!("Hello");
            return None
        },
        Err(_) => None,
    }
}