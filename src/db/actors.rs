use super::models::{Expenses, ModUpdateExpense, User, Group};
use super::schema::expenses::dsl::expenses;
use super::schema::expenses::{cost, name as expense_name, id as expense_id};
use super::schema::user_group::dsl::{user_group};
use super::schema::group::dsl::group;
use super::schema::group::{id as group_id, join_code, name as group_name, owner};
use super::schema::user_group::{group_id as group_in_group_id, user_id as user_in_group_id, user_group_id};
use super::utils::DbActor;
use super::insertables::{NewExpense, NewGroup, NewUser, NewUserGroup};
use super::schema::user::{dsl::{user,first_name,last_name,email,password,username}, id as user_id};
use super::messages::{CreateGroup, CreateUser, DeleteExpense, FetchUser, GetBelongingGroupsName, GetGroupExpenses, GetJoinCode, GetMyExpenses, GetMyGroupName, GetSummedGroupExpenses, GetUserId, InsertExpense, IsUserGroupOwner, JoinGroup, LogIn, UpdateExpense};
use actix::Handler;
use bigdecimal::BigDecimal;
use diesel::dsl::sum;
use diesel::{self, prelude::*, dsl::exists};
use uuid::Uuid;

impl Handler<FetchUser> for DbActor{
    type Result = QueryResult<Vec<User>>;

    fn handle(&mut self, _msg: FetchUser, _ctx: &mut Self::Context) -> Self::Result{
        let mut conn = self.0.get().expect("Fetch User: Unable to establish cnnection");
        user.get_results::<User>(&mut conn)
    }
}

impl Handler<CreateUser> for DbActor{
    type Result = QueryResult<usize>;

    fn handle(&mut self, msg: CreateUser, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get().expect("Create User: Unable to establish connection");

        let new_user = NewUser{
            first_name: msg.first_name,
            last_name: msg.last_name,
            email: msg.email,
            username: msg.username,
            password: msg.password,
        };

        diesel::insert_into(user)
            .values(new_user)
            .returning((
                user_id,
                first_name,
                last_name,
                email,
                username,
                password,
            ))
            .execute(&mut conn)
    }
}

impl Handler<LogIn> for DbActor{
    type Result = QueryResult<bool>;

    fn handle(&mut self, msg: LogIn, _ctx: &mut Self::Context) -> Self::Result{
        let mut conn = self.0.get().expect("Create User: Unable to establish connection");
        diesel::select(exists(
            user.filter(username.eq(msg.username))
                .filter(password.eq(msg.password))
        )).get_result::<bool>(&mut conn)
    }
}

impl Handler<CreateGroup> for DbActor{
    type Result = QueryResult<usize>;

    fn handle(&mut self, msg: CreateGroup, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get().expect("Create User: Unable to establish connection");
        let new_group = NewGroup{
            name: msg.name.clone(),
            owner: msg.owner,
            join_code: Uuid::new_v4(),
        };
        diesel::insert_into(group).values(new_group).execute(&mut conn).expect("coudln't insert group");
        let grop_id = group.filter(group_name.eq(msg.name)).select(group_id).first::<i32>(&mut conn).expect("coudln't get new group from group table");
        
        let new_user_group = NewUserGroup{
            user_id: msg.owner,
            group_id: grop_id 
        };
        diesel::insert_into(user_group).values(new_user_group).execute(&mut conn)
    }
}

impl Handler<GetUserId> for DbActor{
    type Result = QueryResult<i32>;

    fn handle(&mut self, msg: GetUserId, _ctx: &mut Self::Context) -> Self::Result{
        let mut conn = self.0.get().expect("Create User: Unable to establish connection");
        user.filter(username.eq(msg.username)).select(user_id).first::<i32>(&mut conn)
    }
}

impl Handler<GetMyGroupName> for DbActor{
    type Result = QueryResult<Vec<String>>;

    fn handle(&mut self, msg: GetMyGroupName, _ctx: &mut Self::Context) -> Self::Result{
        let mut conn = self.0.get().expect("Create User: Unable to establish connection");
        group.filter(owner.eq(msg.user_id)).select(group_name).get_results(&mut conn)
    }
}

impl Handler<GetBelongingGroupsName> for DbActor{
    type Result = QueryResult<Vec<String>>;

    fn handle(&mut self, msg: GetBelongingGroupsName, _ctx: &mut Self::Context) -> Self::Result{
        let mut conn = self.0.get().expect("Create User: Unable to establish connection");
        group.inner_join(user_group)
             .filter(user_in_group_id.eq(msg.user_id))
             .select(group_name)
             .get_results(&mut conn)
    }
}

impl Handler<GetJoinCode> for DbActor{
    type Result = QueryResult<Uuid>;

    fn handle(&mut self, msg: GetJoinCode, _ctx: &mut Self::Context) -> Self::Result{
        let mut conn = self.0.get().expect("Create User: Unable to establish connection");
        let grop_id: i32 = group.filter(group_name.eq(msg.group_name)).select(group_id).first(&mut conn).expect("Insert Expense: couldn't find group");
        
        group.filter(group_id.eq(grop_id))
             .select(join_code)
             .get_result(&mut conn)
    }
}

impl Handler<JoinGroup> for DbActor{
    type Result = QueryResult<usize>;

    fn handle(&mut self, msg: JoinGroup, _ctx: &mut Self::Context) -> Self::Result{
        let mut conn = self.0.get().expect("Create User: Unable to establish connection");
        let grop_id: i32 = group.filter(join_code.eq(msg.code)).select(group_id).first(&mut conn).expect("Insert Expense: couldn't find group");
        
        let new_user_group = NewUserGroup{
            user_id: msg.user_id,
            group_id: grop_id,
        };

        diesel::insert_into(user_group).values(new_user_group).execute(&mut conn)
    }
}

// Expenses

impl Handler<GetGroupExpenses> for DbActor{
    type Result = QueryResult<Vec<(String,String,BigDecimal,i32)>>;
    fn handle(&mut self, msg: GetGroupExpenses, _ctx: &mut Self::Context) -> Self::Result{
        let mut conn = self.0.get().expect("Create User: Unable to establish connection");

        expenses.inner_join(user_group
                    .inner_join(group)
                    .inner_join(user))
                .filter(group_name.eq(msg.group_name))
                .select((username,expense_name,cost,expense_id)).get_results(&mut conn)
    }
}

impl Handler<GetSummedGroupExpenses> for DbActor{
    type Result = QueryResult<Vec<(String,Option<BigDecimal>)>>;
    fn handle(&mut self, msg: GetSummedGroupExpenses, _ctx: &mut Self::Context) -> Self::Result{
        let mut conn = self.0.get().expect("Create User: Unable to establish connection");
        expenses.inner_join(user_group
                .inner_join(group)
                .inner_join(user))
            .filter(group_name.eq(msg.group_name))
            .group_by(username)
            .select((username, sum(cost)))
            .load::<(String, Option<BigDecimal>)>(&mut conn)
    }
}


impl Handler<GetMyExpenses> for DbActor{
    type Result = QueryResult<Vec<(String,String,BigDecimal,i32)>>;
    fn handle(&mut self, msg: GetMyExpenses, _ctx: &mut Self::Context) -> Self::Result{
        let mut conn = self.0.get().expect("Create User: Unable to establish connection");

        expenses.inner_join(user_group
                    .inner_join(group)
                    .inner_join(user))
                .filter(user_id.eq(msg.user_id))
                .select((username,expense_name,cost,expense_id)).get_results(&mut conn)
    }
}

impl Handler<InsertExpense> for DbActor{
    type Result = QueryResult<usize>;
    fn handle(&mut self, msg: InsertExpense, _ctx: &mut Self::Context) -> Self::Result{
        let mut conn = self.0.get().expect("Create User: Unable to establish connection");
        let grop_id: i32 = group.filter(group_name.eq(msg.group_name)).select(group_id).first(&mut conn).expect("Insert Expense: couldn't find group"); 
        let usr_group_id = user_group
                                .filter(user_in_group_id.eq(msg.user_id))
                                .filter(group_in_group_id.eq(grop_id))
                                .select(user_group_id)
                                .first(&mut conn)
                                .expect("couldn't find usr_group with given group_id + user_id");
        let new_expense = NewExpense{
            name: msg.name,
            cost: msg.cost,
            user_group_id: usr_group_id,
        };
        
        diesel::insert_into(expenses)
            .values(new_expense).execute(&mut conn)
    }
}


impl Handler<UpdateExpense> for DbActor{
    type Result = QueryResult<usize>;
    fn handle(&mut self, msg: UpdateExpense, _ctx: &mut Self::Context) -> Self::Result{
        let mut conn = self.0.get().expect("Create User: Unable to establish connection");      
        let new_expense = ModUpdateExpense{
            id: msg.expense_id,
            name: msg.name,
            cost: msg.cost,
        };
        
        diesel::update(expenses.filter(expense_id.eq(msg.expense_id)))
            .set(new_expense)
            .execute(&mut conn)
    }
}

impl Handler<DeleteExpense> for DbActor{
    type Result = QueryResult<usize>;
    fn handle(&mut self, msg: DeleteExpense, _ctx: &mut Self::Context) -> Self::Result{
        let mut conn = self.0.get().expect("Create User: Unable to establish connection");      
        
        diesel::delete(expenses.filter(expense_id.eq(msg.expense_id)))
            .execute(&mut conn)
    }
}

impl Handler<IsUserGroupOwner> for DbActor{
    type Result = bool;
    fn handle(&mut self, msg: IsUserGroupOwner, _ctx: &mut Self::Context) -> Self::Result{
        let mut conn = self.0.get().expect("Create User: Unable to establish connection");      
        let grop_id: i32 = group.filter(group_name.eq(msg.group_name)).select(group_id).first(&mut conn).expect("Insert Expense: couldn't find group");

        match group.filter(owner.eq(msg.usr_id))
        .filter(group_id.eq(grop_id))
        .select(group_id)
        .get_result::<i32>(&mut conn){
            Ok(_) => true,
            Err(_) => false,
        }
    }
}