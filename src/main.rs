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
use dotenv::dotenv;
use std::env;


mod services;
mod db_utils;
mod messages;
mod actors;
mod db_models;
mod schema;
mod insertables;

use db_utils::{get_pool, AppState, DbActor};
use services::{create_user, echo, get_users, hello, manual_hello};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let db_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool: Pool<ConnectionManager<PgConnection>> = get_pool(&db_url);
    let db_addr = SyncArbiter::start(5, move || DbActor(pool.clone()));

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState{db: db_addr.clone()}))
            .wrap(Cors::default()
                .allowed_origin("http://127.0.0.1:8081") 
                .allowed_methods(vec!["GET", "POST"])    
                .allowed_header(header::CONTENT_TYPE)    
                .max_age(3600))  
            .wrap(Logger::default())
            .service(hello)
            .service(echo)
            .service(get_users)
            .service(create_user)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}