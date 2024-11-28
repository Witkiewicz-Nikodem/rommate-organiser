use actix_web::{Error, HttpResponse};
use actix_session::Session;
use log::info;
use uuid::Uuid;

pub fn log_in(session: Session) -> Result<HttpResponse, Error> {
    session.insert("user_id", Uuid::new_v4().to_string())?;
    Ok(HttpResponse::Ok().into())
}

pub fn log_out(session: Session) -> Result<HttpResponse, Error>{
    if let Some(user_id) = session.get::<String>("user_id")? {
        info!("logout: {:?} ", user_id);
        session.remove("user_id");
        Ok(HttpResponse::Ok().into())
    } else {
        Ok(HttpResponse::BadRequest().into())
    }
}

pub fn is_logged_in(session: &Session) -> Result<bool,Error>{
    if let Some(user_id) = session.get::<String>("user_id")?{
        info!("is_logged_in: {:?} ", user_id);
        Ok(true)
    }
    else {
        Ok(false)
    }
}