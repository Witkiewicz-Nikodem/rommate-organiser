use actix_web::{Error, HttpResponse};
use actix_session::Session;
use log::info;

pub fn log_in(session: Session, user_id: i32) -> Result<HttpResponse, Error> {
    session.insert("user_id", user_id)?;
    Ok(HttpResponse::Ok().into())
}

pub fn log_out(session: Session) -> Result<HttpResponse, Error>{
    if let Some(_) = session.get::<i32>("user_id")? {
        session.remove("user_id");
        Ok(HttpResponse::Ok().into())
    } else {
        Ok(HttpResponse::BadRequest().into())
    }
}

pub fn is_logged_in(session: &Session) -> bool{
    if let Ok(Some(_)) = session.get::<i32>("user_id"){
        true
    }
    else {
        false
    }
}

pub fn get_id(session: &Session) -> Option<i32>{
    match session.get::<i32>("user_id"){
        Ok(user_id) => user_id,
        Err(err) => {
            info!("user Not Found in session, err: {:?}", err);
            None
        }
    }
}