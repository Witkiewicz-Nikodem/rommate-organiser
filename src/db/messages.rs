use super::models::{User, Group};
use actix::Message;
use diesel::QueryResult;
use serde::Deserialize;
use bigdecimal::BigDecimal;
use uuid::Uuid;

#[derive(Message)]
#[rtype(result = "QueryResult<Vec<User>>")]
pub struct FetchUser;

#[derive(Message)]
#[rtype(result= "QueryResult<Vec<String>>")]
pub struct GetMyGroupName{
    pub user_id: i32,
}

#[derive(Message)]
#[rtype(result= "QueryResult<Vec<String>>")]
pub struct GetBelongingGroupsName{
    pub user_id: i32,
}


#[derive(Message)]
#[rtype(result= "QueryResult<usize>")]
pub struct CreateUser{
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Message)]
#[rtype(result= "QueryResult<usize>")]
pub struct CreateGroup{
    pub name: String,
    pub owner: i32,
}

#[derive(Message)]
#[rtype(result= "QueryResult<Uuid>")]
pub struct GetJoinCode{
    pub group_name: String,
}

#[derive(Message)]
#[rtype(result= "bool")]
pub struct IsUserGroupOwner{
    pub group_name: String,
    pub usr_id: i32
}

#[derive(Message,Deserialize)]
#[rtype(result= "QueryResult<usize>")]
pub struct JoinGroup{
    pub code: Uuid,
    pub user_id: i32,
}

#[derive(Message,Deserialize)]
#[rtype(result= "QueryResult<usize>")]
pub struct DeleteGroup{
    pub group_name: String,
}

#[derive(Message,Deserialize)]
#[rtype(result= "QueryResult<usize>")]
pub struct PutNewName{
    pub old_name: String,
    pub new_name: String,
}

#[derive(Message, Deserialize, Clone)]
#[rtype(result = "QueryResult<bool>")]
pub struct LogIn{
    pub username: String,
    pub password: String,
}

#[derive(Message)]
#[rtype(result= "QueryResult<i32>")]
pub struct GetUserId{
    pub username: String,
}


// Expenses

#[derive(Message)]
#[rtype(result= "QueryResult<Vec<(String,String,BigDecimal,i32)>>")]
pub struct GetGroupExpenses{
    pub group_name: String,
}

#[derive(Message)]
#[rtype(result= "QueryResult<Vec<(String,String,BigDecimal,i32)>>")]
pub struct GetMyExpenses{
    pub user_id: i32,
}

#[derive(Message)]
#[rtype(result= "QueryResult<Vec<(String,Option<BigDecimal>)>>")]
pub struct GetSummedGroupExpenses{
    pub group_name: String,
}

#[derive(Message, Clone)]
#[rtype(result= "QueryResult<(usize)>")]
pub struct InsertExpense{
    pub group_name: String,
    pub user_id: i32,
    pub name: String,
    pub cost: BigDecimal,
}

#[derive(Message)]
#[rtype(result= "QueryResult<(usize)>")]
pub struct UpdateExpense{
    pub name: String,
    pub cost: BigDecimal,
    pub expense_id: i32
}

#[derive(Message)]
#[rtype(result= "QueryResult<(usize)>")]
pub struct DeleteExpense{
    pub expense_id: i32
}

// HTML
#[derive(Message)]
#[rtype(result = "QueryResult<String>")]
pub struct GetHTML{
    pub html_object_name: String, 
}