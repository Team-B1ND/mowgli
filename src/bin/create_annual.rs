use crate::models::{NewUser, User};
use crate::schema::users::dsl::users;
use diesel::prelude::*;
use mowgli::establish_connection;

pub fn create_annual(id: i64) {
    let connection = &mut establish_connection();
    let new_user = NewUser { id: &id, annual: &0 };
    diesel::insert_into(crate::schema::users::table)
        .values(&new_user)
        .execute(connection);
}