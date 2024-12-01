use crate::{db::schema::{group, user_group}, schema::{expenses, user}};
use bigdecimal:: BigDecimal;
use diesel::Insertable;
use serde::Serialize;
use uuid::Uuid;

#[derive(Insertable, Serialize,Clone)]
#[diesel(table_name=user)]
pub struct NewUser{
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
    pub password: String,
}


#[derive(Insertable, Serialize,Clone)]
#[diesel(table_name=group)]
pub struct NewGroup{
    pub name: String,
    pub owner: i32, 
    pub join_code: Uuid,
}

#[derive(Insertable, Serialize,Clone)]
#[diesel(table_name=user_group)]
pub struct NewUserGroup{
    pub user_id: i32, 
    pub group_id: i32
}

#[derive(Insertable, Serialize,Clone)]
#[diesel(table_name=expenses)]
pub struct NewExpense{
    pub name: String,
    pub cost: BigDecimal,
    pub user_group_id: i32,
}

