use super::models::User;
use super::schema::expenses::dsl::expenses;
use super::schema::expenses::{cost, name as expense_name, id as expense_id};
use super::schema::user_group::dsl::{user_group};
use super::schema::group::dsl::group;
use super::schema::group::{name as group_name, owner};
use super::schema::user_group::user_id as user_in_group_id;
use super::utils::DbActor;
use super::insertables::{NewGroup, NewUser};
use super::schema::user::{dsl::{user,first_name,last_name,email,password,username}, id as user_id};
use super::messages::{CreateGroup, CreateUser, FetchUser, GetBelongingGroupsName, GetGroupExpenses, GetMyExpenses, GetMyGroupName, GetUserId, LogIn};
use actix::Handler;
use bigdecimal::BigDecimal;
use diesel::{self, prelude::*, dsl::exists};

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
            name: msg.name,
            owner: msg.owner,
        };
        diesel::insert_into(group).values(new_group).execute(&mut conn)
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