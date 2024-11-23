use crate::db_models::{User};
use actix::Message;
use diesel::QueryResult;


#[derive(Message)]
#[rtype(result = "QueryResult<Vec<User>>")]
pub struct FetchUser;

#[derive(Message)]
#[rtype(result= "QueryResult<User>")]
pub struct CreateUser{
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
    pub password: String,
}