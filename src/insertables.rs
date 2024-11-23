use crate::schema::users;
use diesel::Insertable;
use serde::Serialize;

#[derive(Insertable, Serialize,Clone)]
#[diesel(table_name=users)]
pub struct NewUser{
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
    pub password: String,
}