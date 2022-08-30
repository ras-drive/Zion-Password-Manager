use diesel::prelude::*;
use crate::schema::logins;

#[derive(Queryable, Debug, PartialEq)]
pub struct Login {
    pub id: i32,
    pub email: String,
    pub password_hash: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = logins)]
pub struct NewLogin<'a> {
    pub id: i32,
    pub email: &'a str,
    pub password_hash: &'a str,
}
