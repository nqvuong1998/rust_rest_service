use actix::SyncArbiter;
use actix_web::{web::Data, App, HttpServer};
use dotenv::dotenv;
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection
};
use std::env;

mod services;
mod db_utils;
mod messages;
mod actors;
mod db_models;
mod schema;
mod insertables;

use db_utils::{get_pool, AppState, DbActor};
use services::{create_user,update_user,get_user,create_post,update_post,delete_post,get_post,get_posts};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool: Pool<ConnectionManager<PgConnection>> = get_pool(&db_url);
    let db_addr = SyncArbiter::start(5, move || DbActor(pool.clone()));
    
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState { db: db_addr.clone() }))
            .service(create_user)
            .service(update_user)
            .service(get_user)
            .service(create_post)
            .service(update_post)
            .service(delete_post)
            .service(get_post)
            .service(get_posts)
    })
    .bind(("127.0.0.1", 9999))?
    .run()
    .await
}
