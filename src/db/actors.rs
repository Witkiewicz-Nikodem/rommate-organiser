use super::models::User;
use super::schema::group::dsl::{group,id as group_id};
use super::schema::group::{name, owner};
use super::utils::DbActor;
use super::insertables::{NewGroup, NewUser};
use super::schema::user::{dsl::{user,first_name,last_name,email,password,username}, id as user_id};
use super::messages::{CreateGroup, CreateUser, FetchUser, GetGroup, GetUserId, LogIn};
use actix::Handler;
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

impl Handler<GetGroup> for DbActor{
    type Result = QueryResult<Vec<String>>;

    fn handle(&mut self, msg: GetGroup, _ctx: &mut Self::Context) -> Self::Result{
        let mut conn = self.0.get().expect("Create User: Unable to establish connection");
        group.filter(owner.eq(msg.user_id)).select(name).get_results(&mut conn)
    }
}

