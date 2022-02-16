use std::sync::RwLock;
use crate::database::DbConnection;
use crate::user::User;
use actix_web::web::Data;
use actix_session::CookieSession;
use actix_web::{middleware, App, HttpServer};
use std::io;

pub mod api;
pub mod database;
pub mod user;

struct ServerContext {
    db: DbConnection,
    users: RwLock<Vec<User>>,
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let db = DbConnection::new("http://127.0.0.1:5984", "admin", "password").await.unwrap();
    db.setup_db().await;

    let context = Data::new(ServerContext {
        db: db,
        users: RwLock::new(vec![]),
    });

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            // enable automatic response compression - usually register this first
            .wrap(middleware::Compress::default())
            // cookie session middleware
            .wrap(CookieSession::signed(&[0; 32]).secure(true))
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            // register simple route, handle all methods
            .app_data(context.clone())
            .service(api::server::banner)
    })
    .bind(("127.0.0.1", 8080))?
    .workers(2)
    .run()
    .await
}
