use actix_web::{
    get, post,
    web::{Data, Json, Path},
    Responder,
    HttpResponse
};
use serde::Deserialize;
use crate::{
    messages::{CreateUser, FetchUser},
    AppState, DbActor
};
use actix::Addr;

#[derive(Deserialize)]
pub struct CreateUserBody{
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
    pub password: String,
}



#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
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

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[post("/users")]
pub async fn create_user(state: Data<AppState>, body: Json<CreateUserBody>) -> impl Responder{
    let db: Addr<DbActor> = state.as_ref().db.clone();
    match db.send(CreateUser{
        id: body.id,
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