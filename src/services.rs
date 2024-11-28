use actix_session::Session;
use actix_web::{
    get, post, delete, put,
    web::{Data, Json, Path},
    Responder,
    HttpResponse
};
use serde::Deserialize;
use crate::{
    messages::{CreateUser, FetchUser, LogIn},
    AppState, DbActor,
    session,
};
use actix::Addr;
use log::info;

#[derive(Deserialize,Debug)]
pub struct CreateUserBody{
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
    pub password: String,
}


#[get("/users")]
pub async fn get_users(state: Data<AppState>) -> impl Responder{
    let db: Addr<DbActor> = state.as_ref().db.clone();

    match db.send(FetchUser).await{
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(_)) => HttpResponse::NotFound().json("No users found"),
        _ => HttpResponse::InternalServerError().json("Unable to retrieve users"),
    }
}

#[post("/user")]
pub async fn create_user(state: Data<AppState>, body: Json<CreateUserBody>) -> impl Responder{
    let db: Addr<DbActor> = state.as_ref().db.clone();
    info!("Post User Body: {:?}",body);
    match db.send(CreateUser{
        first_name: body.first_name.to_string(),
        last_name: body.last_name.to_string(),
        email: body.email.to_string(),
        username: body.username.to_string(),
        password: body.password.to_string(),
    }).await
    {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        _ => HttpResponse::InternalServerError().json("Failed to create User"),
    }
}

#[post("/log_in")]
pub async fn log_in(state: Data<AppState>, body:Json<LogIn>, session: Session) -> impl Responder{
    let db: Addr<DbActor> = state.as_ref().db.clone();
    match db.send(body.clone()).await {
        Ok(Ok(true)) => {
            match session::log_in(session){
                Ok(response) => response,
                Err(_error) => HttpResponse::InternalServerError().json("Failed to Log in"),
            }
        }
        _            => HttpResponse::InternalServerError().json("failed to Log in"),
    }
}

#[delete("/log_out")]
pub async fn log_out(session: Session) -> impl Responder{   
    match session::is_logged_in(&session){
        Ok(true) =>{
            match session::log_out(session){
                Ok(response) => response,
                Err(_error) => HttpResponse::InternalServerError().json("Failed to Log in"),
       }
       }
        Ok(false) => HttpResponse::BadRequest().json("your are not loged in"),
        Err(_error) => HttpResponse::InternalServerError().json("Failed to Log out")
    }
}