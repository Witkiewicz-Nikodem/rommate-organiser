use super::models::{User};
use actix::Message;
use diesel::QueryResult;
use serde::Deserialize;

#[derive(Message)]
#[rtype(result = "QueryResult<Vec<User>>")]
pub struct FetchUser;

#[derive(Message)]
#[rtype(result= "QueryResult<User>")]
pub struct CreateUser{
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Message, Deserialize, Clone)]
#[rtype(result = "QueryResult<bool>")]
pub struct LogIn{
    pub username: String,
    pub password: String,
}