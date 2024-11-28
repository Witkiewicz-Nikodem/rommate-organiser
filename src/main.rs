use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_web::http::header;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_cors::Cors;
use actix::SyncArbiter;
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection
};
use actix_session::config::PersistentSession;
use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use actix_web::cookie::time::Duration;
use actix_web::cookie::Key;
use dotenv::dotenv;
use std::env;
use env_logger;

mod db;
mod services;
mod sessions;

use crate::db::actors;
use crate::db::insertables;
use crate::db::messages;
use crate::db::models;
use crate::db::schema;
use crate::db::utils;
use crate::sessions::session;

use utils::{get_pool, AppState, DbActor};
use services::{get_users, create_user, log_in, log_out};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    
    

    let db_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool: Pool<ConnectionManager<PgConnection>> = get_pool(&db_url);
    let db_addr = SyncArbiter::start(5, move || DbActor(pool.clone()));

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(
                // create cookie based session middleware
                SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&[0; 64]))
                    .session_lifecycle(PersistentSession::default().session_ttl(Duration::minutes(30)))
                    .cookie_secure(false)
                    .build()
            )
            .app_data(Data::new(AppState{db: db_addr.clone()}))
            .wrap(Cors::default()
                .allowed_origin("http://192.168.56.1:8081") 
                .allowed_methods(vec!["GET", "POST"])
                .allowed_header(header::CONTENT_TYPE)
                .max_age(3600))  
            .service(get_users)
            .service(create_user)
            .service(log_in)
            .service(log_out)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}