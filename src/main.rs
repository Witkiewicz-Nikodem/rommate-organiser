use std::path::Path;
use actix_web::{App, HttpServer};
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
use actix_web::cookie::{Key, SameSite};
use dotenv::dotenv;
use html_synthesis::assembly_page::{get_logged_in_create_group, get_logged_in_home, get_logged_in_join_group, get_logged_in_manage_owned_groups, get_logged_in_manage_owned_groups_specific, get_logged_in_my_groups, get_logged_in_my_groups_specyfic, get_logged_in_support, get_logged_out_index, get_logged_out_login, get_logged_out_register, get_logged_out_support};
use std::env;
use env_logger;

mod db;
mod services;
mod sessions;
mod io_api_schemes;
mod html_synthesis;

use crate::db::messages;
use crate::db::schema;
use crate::db::utils;
use crate::sessions::session;
use crate::html_synthesis::{getters,builder,assembly_page};

use utils::{get_pool, AppState, DbActor};
use services::{create_group, create_user, delete_expense, delete_group, get_belonging_groups, get_groups_expenses, get_join_code, get_my_expenses, get_my_groups, get_summed_groups_expenses, get_users, insert_expense, log_in, log_out, post_join_group, put_group, update_expense};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    let port = env::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(8080);
    let address = format!("127.0.0.1:{}", port);

    let db_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool: Pool<ConnectionManager<PgConnection>> = get_pool(&db_url);
    let db_addr = SyncArbiter::start(5, move || DbActor(pool.clone()));

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(
                // create cookie based session middleware
                SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&[0; 64]))
                    .session_lifecycle(PersistentSession::default().session_ttl(Duration::minutes(15)))
                    .cookie_secure(false)
                    .cookie_http_only(false)
                    .cookie_same_site(SameSite::Lax)
                    .build()
            )
            .app_data(Data::new(AppState{db: db_addr.clone()}))
            .wrap(Cors::default()
                .allowed_origin("http://192.168.56.1:8081") 
                .allowed_origin("http://127.0.0.1:8080") 
                .allowed_origin("http://localhost:8080") 
                .allowed_origin("https://rust-app-production.up.railway.app")
                .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                .allowed_headers(vec![header::CONTENT_TYPE, header::SET_COOKIE])
                .max_age(3600))  
            .service(get_users)
            .service(create_user)
            .service(log_in)
            .service(log_out)
            .service(create_group)
            .service(get_my_groups)
            .service(get_belonging_groups) 
            .service(get_groups_expenses)
            .service(get_summed_groups_expenses)
            .service(get_my_expenses)
            .service(insert_expense)
            .service(update_expense)
            .service(delete_expense)
            .service(get_join_code)
            .service(post_join_group)
            .service(delete_group)
            .service(put_group)
            .service(actix_files::Files::new(
                "/static",
                Path::new("./static")
            
            ))
            .service(get_logged_out_index)
            .service(get_logged_out_support)
            .service(get_logged_out_register)
            .service(get_logged_out_login)
            .service(get_logged_in_home)
            .service(get_logged_in_support)
            .service(get_logged_in_my_groups)
            .service(get_logged_in_my_groups_specyfic)
            .service(get_logged_in_manage_owned_groups)
            .service(get_logged_in_manage_owned_groups_specific)
            .service(get_logged_in_join_group)
            .service(get_logged_in_create_group)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}