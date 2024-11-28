use crate::schema::user;
use diesel::Insertable;
use serde::Serialize;

#[derive(Insertable, Serialize,Clone)]
#[diesel(table_name=user)]
pub struct NewUser{
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
    pub password: String,
}