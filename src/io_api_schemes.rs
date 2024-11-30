use serde::Deserialize;

#[derive(Deserialize,Debug)]
pub struct CreateUserBody{
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Deserialize,Debug)]
pub struct CreateGroupBody{
    pub name: String,
}