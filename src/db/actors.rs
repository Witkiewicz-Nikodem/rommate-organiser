use super::models::User;
use super::utils::DbActor;
use super::insertables::NewUser;
use super::schema::user::{dsl::*, id as user_id};
use super::messages::{FetchUser, CreateUser, LogIn};
use actix::Handler;
use diesel::{self, prelude::*, dsl::exists};
use log::info;

impl Handler<FetchUser> for DbActor{
    type Result = QueryResult<Vec<User>>;

    fn handle(&mut self, _msg: FetchUser, _ctx: &mut Self::Context) -> Self::Result{
        let mut conn = self.0.get().expect("Fetch User: Unable to establish cnnection");
        user.get_results::<User>(&mut conn)
    }
}

impl Handler<CreateUser> for DbActor{
    type Result = QueryResult<User>;

    fn handle(&mut self, msg: CreateUser, ctx: &mut Self::Context) -> Self::Result {
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
            .get_result::<User>(&mut conn)
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