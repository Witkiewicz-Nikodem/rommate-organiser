use actix_session::Session;
use actix_web::{
    delete, get, post, web::{self, Data, Json}, HttpResponse, Responder
};
use crate::{
    db::messages::{CreateGroup, GetGroupExpenses, GetGroupName, GetUserId}, io_api_schemes::{CreateGroupBody, CreateUserBody}, messages::{CreateUser, FetchUser, LogIn}, session, AppState, DbActor
};
use actix::Addr;
use log::info;


#[get("/users")]
pub async fn get_users(state: Data<AppState>) -> impl Responder{
    let db: Addr<DbActor> = state.as_ref().db.clone();

    match db.send(FetchUser).await{
        Ok(Ok(response)) => HttpResponse::Ok().json(response),
        Ok(Err(_)) => HttpResponse::NotFound().json("No users found"),
        _ => HttpResponse::InternalServerError().json("Unable to retrieve users"),
    }
}


#[get("/expenses/{name}")]
pub async fn get_groups_expenses(state: Data<AppState>, session: Session, name: web::Path<String>) -> impl Responder{
    let db: Addr<DbActor> = state.as_ref().db.clone();

    match session::is_logged_in(&session){
        true => {
            match db.send(GetGroupExpenses{group_name: name.to_string()}).await{
                Ok(Ok(response)) => HttpResponse::Ok().json(response),
                Ok(Err(_)) => HttpResponse::NotFound().json("No users found"),
                _ => HttpResponse::InternalServerError().json("Unable to retrieve users"),
            }
        }
        false => HttpResponse::InternalServerError().json("u must be logged in to get yours groups")
    }
}

#[get("/groups")]
pub async fn get_groups(state: Data<AppState>, session: Session) -> impl Responder{
    let db: Addr<DbActor> = state.as_ref().db.clone();

    match session::get_id(&session) {
        Some(user_id) =>{
            match db.send(GetGroupName{user_id}).await{
                Ok(Ok(response)) => HttpResponse::Ok().json(response),
                Ok(Err(_)) => HttpResponse::NotFound().json("No groups found"),
                _ => HttpResponse::InternalServerError().json("Unable to retrieve users"),
            }
        }
        None          => HttpResponse::InternalServerError().json("u must be logged in to get yours groups")
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
        Ok(Ok(_)) => HttpResponse::Ok(),
        _ => HttpResponse::InternalServerError(),
    }
}

#[post("/group")]
pub async fn create_group(state: Data<AppState>, body: Json<CreateGroupBody>, session: Session) -> impl Responder{
    let db: Addr<DbActor> = state.as_ref().db.clone();
    match session::get_id(&session) {
        Some(user_id) =>{
            match db.send(CreateGroup{
                name: body.name.to_string(),
                owner: user_id,
            }).await
            {
                Ok(Ok(_)) => HttpResponse::Ok().finish(),
                err => {info!("create group error: {:?}",err);HttpResponse::InternalServerError().finish()},
            }
        }
        None          => HttpResponse::InternalServerError().json("u must be logged in to create Group")
    }
}


#[post("/log_in")]
pub async fn log_in(state: Data<AppState>, body:Json<LogIn>, session: Session) -> impl Responder{
    let db: Addr<DbActor> = state.as_ref().db.clone();
    match db.send(body.clone()).await {
        Ok(Ok(true)) => {
            let user_id = match db.send(GetUserId{username: body.username.clone()}).await{
                Ok(Ok(id)) => id,
                Err(e) => {
                    info!("function Log_in -> couldn't read userId from result GetUserId, error: {:?}", e);
                    return HttpResponse::InternalServerError()
                },    
                _      => return HttpResponse::InternalServerError()
            };         
            match session::log_in(session, user_id){
                Ok(_)       => HttpResponse::Ok(),
                Err(_error) => HttpResponse::InternalServerError(),
            }
        }
        _            => HttpResponse::InternalServerError(),
    }
}

#[delete("/log_out")]
pub async fn log_out(session: Session) -> impl Responder{   
    match session::is_logged_in(&session){
        true =>{
            match session::log_out(session){
                Ok(response) => response,
                Err(_error) => HttpResponse::InternalServerError().json("Failed to Log in"),
            }
        }
        false => HttpResponse::BadRequest().json("your are not loged in"),
    }
}



