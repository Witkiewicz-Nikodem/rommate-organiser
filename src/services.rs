use actix_session::Session;
use actix_web::{
    delete, get, post, put, web::{self, Data, Json}, HttpResponse, Responder
};
use crate::{
    db::messages::{CreateGroup, DeleteExpense, DeleteGroup, GetBelongingGroupsName, GetGroupExpenses, GetJoinCode, GetMyExpenses, GetMyGroupName, GetSummedGroupExpenses, GetUserId, InsertExpense, IsUserGroupOwner, JoinGroup, PutNewName, UpdateExpense},
    io_api_schemes::{CreateGroupBody, CreateUserBody, InsertExpenseBody, JoinGroupBody, UpdateExpenseBody},
    messages::{CreateUser, FetchUser, LogIn},
    session, AppState, DbActor
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

#[get("/my/groups")]
pub async fn get_my_groups(state: Data<AppState>, session: Session) -> impl Responder{
    let db: Addr<DbActor> = state.as_ref().db.clone();

    match session::get_id(&session) {
        Some(user_id) =>{
            match db.send(GetMyGroupName{user_id}).await{
                Ok(Ok(response)) => HttpResponse::Ok().json(response),
                Ok(Err(_)) => HttpResponse::NotFound().json("No groups found"),
                _ => HttpResponse::InternalServerError().json("Unable to retrieve users"),
            }
        }
        None          => HttpResponse::Unauthorized().json("u must be logged in to get yours groups")
    }
}

#[get("belonging/groups")]
pub async fn get_belonging_groups(state: Data<AppState>, session: Session) -> impl Responder{
    let db: Addr<DbActor> = state.as_ref().db.clone();
    match session::get_id(&session) {
        Some(user_id) =>{
            match db.send(GetBelongingGroupsName{user_id}).await{
                Ok(Ok(response)) => HttpResponse::Ok().json(response),
                Ok(Err(_)) => HttpResponse::NotFound().json("No groups found"),
                _ => HttpResponse::InternalServerError().json("Unable to retrieve users"),
            }
        }
        None          => HttpResponse::Unauthorized().json("u must be logged in to get yours groups")
    }
}

#[get("group/join_code/{group_name}")]
pub async fn get_join_code(state: Data<AppState>, session: Session, group_name: web::Path<String>) -> impl Responder{
    let db: Addr<DbActor> = state.as_ref().db.clone();

    let user_id = match session::get_id(&session){
        Some(result) => result,
        None => return HttpResponse::Unauthorized().json("u must be logged in to get yours groups")
    };

    match db.send(IsUserGroupOwner{group_name: group_name.to_string(), usr_id: user_id}).await{
        Ok(true) => {
            match db.send(GetJoinCode{group_name: group_name.to_string()}).await{
                Ok(Ok(response)) => HttpResponse::Ok().json(response),
                Ok(Err(_)) => HttpResponse::NotFound().json("No code for provided data"),
                _ => HttpResponse::InternalServerError().json("Unable to retrieve users"),
            }
        }
        Ok(false) => HttpResponse::BadRequest().json("provided Wrong Data "),
        Err(_) => HttpResponse::InternalServerError().json("Unable check provided data"),
    }
}

#[post("group/join_group")]
pub async fn post_join_group(state: Data<AppState>, session: Session, body: Json<JoinGroupBody>) -> impl Responder{
    let db: Addr<DbActor> = state.as_ref().db.clone();
    match session::get_id(&session){
        Some(user_id) => {
            match db.send(JoinGroup{code: body.code, user_id}).await{
                Ok(Ok(response)) => HttpResponse::Ok().json(response),
                Ok(Err(_)) => HttpResponse::BadRequest().json("Provided Wrong Data"),
                _ => HttpResponse::InternalServerError().json("Unable to retrieve users"),
            }
        }
        None => HttpResponse::Unauthorized().json("your are not loged in"),
    }
}

#[delete("/group/{group_name}")]
pub async fn delete_group(state: Data<AppState>, session: Session, group_name: web::Path<String>) -> impl Responder{
    let db: Addr<DbActor> = state.as_ref().db.clone();

    let user_id = match session::get_id(&session){
        Some(result) => result,
        None => return HttpResponse::Unauthorized().json("u must be logged in to get yours groups")
    };

    match db.send(IsUserGroupOwner{group_name: group_name.to_string(), usr_id: user_id}).await{
        Ok(true) => {
            match db.send(DeleteGroup{group_name: group_name.to_string()}).await{
                Ok(Ok(response)) => HttpResponse::Ok().json(response),
                Ok(Err(_)) => HttpResponse::NotFound().json("No group for provided name"),
                _ => HttpResponse::InternalServerError().json("Unable to retrieve group"),
            }
        }
        Ok(false) => HttpResponse::BadRequest().json("provided Wrong Data "),
        Err(_) => HttpResponse::InternalServerError().json("Unable check provided data"),
    }
}

#[put("/group")]
pub async fn put_group(state: Data<AppState>, session: Session, body: Json<PutNewName>) -> impl Responder{
    let db: Addr<DbActor> = state.as_ref().db.clone();

    match session::is_logged_in(&session){
        true => {
            match db.send(PutNewName{old_name: body.old_name.clone(), new_name: body.new_name.clone()}).await{
                Ok(Ok(response)) => HttpResponse::Ok().json(response),
                Ok(Err(_)) => HttpResponse::NotFound().json("No group for provided name"),
                _ => HttpResponse::InternalServerError().json("Unable to retrieve group"),
            }
        },
        false => return HttpResponse::Unauthorized().json("u must be logged in to get yours groups")
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
        None          => HttpResponse::Unauthorized().json("u must be logged in to create Group")
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
        false => HttpResponse::Unauthorized().json("your are not loged in"),
    }
}

#[get("/expenses/group/{group_name}")]
pub async fn get_groups_expenses(state: Data<AppState>, session: Session, group_name: web::Path<String>) -> impl Responder{
    let db: Addr<DbActor> = state.as_ref().db.clone();

    match session::is_logged_in(&session){
        true => {
            match db.send(GetGroupExpenses{group_name: group_name.to_string()}).await{
                Ok(Ok(response)) => HttpResponse::Ok().json(response),
                Ok(Err(_)) => HttpResponse::NotFound().json("No users found"),
                _ => HttpResponse::InternalServerError().json("Unable to retrieve users"),
            }
        }
        false => HttpResponse::Unauthorized().json("u must be logged in to get yours groups")
    }
}

#[get("/expenses/group/summed/{group_name}")]
pub async fn get_summed_groups_expenses(state: Data<AppState>, session: Session, group_name: web::Path<String>) -> impl Responder{
    let db: Addr<DbActor> = state.as_ref().db.clone();

    match session::is_logged_in(&session){
        true => {
            match db.send(GetSummedGroupExpenses{group_name: group_name.to_string()}).await{
                Ok(Ok(response)) => HttpResponse::Ok().json(response),
                Ok(Err(_)) => HttpResponse::NotFound().json("No users found"),
                _ => HttpResponse::InternalServerError().json("Unable to retrieve users"),
            }
        }
        false => HttpResponse::Unauthorized().json("u must be logged in to get yours groups")
    }
}

#[get("/expenses/user")]
pub async fn get_my_expenses(state: Data<AppState>, session: Session) -> impl Responder{
    let db: Addr<DbActor> = state.as_ref().db.clone();

    match session::get_id(&session) {
        Some(usr_id) =>{
            match db.send(GetMyExpenses{user_id: usr_id}).await{
                Ok(Ok(response)) => HttpResponse::Ok().json(response),
                Ok(Err(_)) => HttpResponse::NotFound().json("No users found"),
                _ => HttpResponse::InternalServerError().json("Unable to retrieve users"),
            }
        }
        None => HttpResponse::Unauthorized().json("u must be logged in to get yours groups")
    }
}

#[post("/expense")]
pub async fn insert_expense(state: Data<AppState>, session: Session, body: Json<InsertExpenseBody>) -> impl Responder{
    let db: Addr<DbActor> = state.as_ref().db.clone();
    match session::get_id(&session) {
        Some(usr_id) =>{
            match db.send(InsertExpense{
                    group_name: body.group_name.clone(),
                    user_id: usr_id,
                    name: body.name.clone(),
                    cost: body.cost.clone(),
                    }).await{
                Ok(Ok(response)) => HttpResponse::Ok().json(response),
                Ok(Err(_)) => HttpResponse::NotFound().json("No users found"),
                _ => HttpResponse::InternalServerError().json("Unable to retrieve users"),
            }
        }
        None => HttpResponse::Unauthorized().json("u must be logged in to get yours groups")
    }
}

#[put("/expense")]
pub async fn update_expense(state: Data<AppState>, session: Session, body: Json<UpdateExpenseBody>) -> impl Responder{
    let db: Addr<DbActor> = state.as_ref().db.clone();
    match session::get_id(&session) {
        Some(user_id) =>{
            match db.send(UpdateExpense{
                    user_id,
                    name: body.name.clone(),
                    cost: body.cost.clone(),
                    expense_id: body.expense_id.clone()
                    }).await{
                Ok(Ok(response)) => HttpResponse::Ok().json(response),
                Ok(Err(e)) => HttpResponse::BadRequest().json(format!("{:?}",e)),
                _ => HttpResponse::InternalServerError().json("Unable to retrieve users"),
            }
        }
        None => HttpResponse::Unauthorized().json("u must be logged in to get yours groups")
    }
}

#[delete("/expense/{expense_id}")]
pub async fn delete_expense(state: Data<AppState>, session: Session, expense_id: web::Path<String>) -> impl Responder{
    let db: Addr<DbActor> = state.as_ref().db.clone();
    let exp_id: i32 = match expense_id.parse::<i32>(){
        Ok(value) => value,
        Err(err) => return HttpResponse::InternalServerError().json(format!("Unable to parse expense_id. err: {:?}", err))
    };
    match session::get_id(&session) {
        Some(user_id) =>{
            match db.send(DeleteExpense{user_id, expense_id: exp_id}).await{
                Ok(Ok(response)) => HttpResponse::Ok().json(response),
                Ok(Err(e)) => HttpResponse::BadRequest().json(format!("{:?}",e)),
                _ => HttpResponse::InternalServerError().json("Unable to retrieve users"),
            }
        }
        None => HttpResponse::Unauthorized().json("u must be logged in to get yours groups")
    }
}
