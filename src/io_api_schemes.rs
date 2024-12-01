use bigdecimal::BigDecimal;
use serde::Deserialize;
use uuid::Uuid;

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

#[derive(Deserialize,Debug)]
pub struct InsertExpenseBody{
    pub group_name: String,
    pub name: String,
    pub cost: BigDecimal
}

#[derive(Deserialize,Debug)]
pub struct UpdateExpenseBody{
    pub name: String,
    pub cost: BigDecimal,
    pub expense_id: i32
}

#[derive(Deserialize,Debug)]
pub struct JoinGroupBody{
    pub code: Uuid,
}