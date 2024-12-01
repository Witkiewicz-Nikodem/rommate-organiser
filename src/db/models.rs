// Generated by diesel_ext

#![allow(unused)]
#![allow(clippy::all)]

use bigdecimal::BigDecimal;
use diesel::prelude::{AsChangeset, QueryableByName};
use diesel::{Identifiable, Queryable};
use serde::Serialize;
use uuid::Uuid;
use chrono::NaiveDateTime;
use diesel::Selectable;


use crate::db::schema;
use crate::schema::expenses::dsl::expenses;
use crate::schema::group::dsl::group;
use crate::schema::user::dsl::user;
use crate::schema::user_group::dsl::user_group;

#[derive(QueryableByName, Debug,Serialize, AsChangeset)]
#[diesel(table_name = schema::expenses)]
pub struct Expenses {
    pub id: i32,
    pub name: String,
    pub cost: BigDecimal,
    pub user_group_id: i32,
}


#[derive(Queryable, Debug,Serialize)]
#[diesel(belongs_to(UserGroup))]
#[diesel(table_name = schema::group)]
pub struct Group {
    pub id: i32,
    pub name: String,
}

#[derive(Queryable, Debug, Serialize)]
#[diesel(belongs_to(UserGroup))]
#[diesel(table_name = schema::user)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
    pub password: String,
}


#[derive(Queryable, Debug, Serialize)]
#[diesel(table_name = schema::user_group)]
#[diesel(belongs_to(Expenses))]
#[diesel(primary_key(user_group_id))]
pub struct UserGroup {
    pub user_group_id: i32,
    pub user_id: i32,
    pub group_id: i32,
}



// not strict models:
#[derive(Queryable, Debug,Serialize, AsChangeset)]
#[diesel(table_name = schema::expenses)]
pub struct ModUpdateExpense {
    pub id: i32,
    pub name: String,
    pub cost: BigDecimal,
}