use crate::{db::schema::group, schema::user, schema::expenses};
use bigdecimal::{num_bigint::BigInt, BigDecimal};
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


#[derive(Insertable, Serialize,Clone)]
#[diesel(table_name=group)]
pub struct NewGroup{
    pub name: String,
    pub owner: i32, 
}

#[derive(Insertable, Serialize,Clone)]
#[diesel(table_name=expenses)]
pub struct NewExpense{
    pub name: String,
    pub cost: BigDecimal,
    pub user_group_id: i32,
}
