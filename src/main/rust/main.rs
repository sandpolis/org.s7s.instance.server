use crate::banner::Banner;
use crate::connection::Connection;
use crate::database::DbConnection;
use crate::user::User;
use actix_session::CookieSession;
use actix_web::web::Data;
use actix_web::{middleware, App, HttpServer};
use rustls::server::AllowAnyAuthenticatedClient;
use rustls::ServerConfig;
use std::io;
use std::sync::RwLock;

pub mod api {
    pub mod v1 {
        pub mod agent {
            pub mod reboot;
        }
        pub mod server {
            pub mod banner;
            pub mod session;
            pub mod users;
        }
        pub mod util;
    }
}
pub mod banner;
pub mod connection;
pub mod database;
pub mod group;
pub mod server;
pub mod user;

struct ServerContext {
    db: DbConnection,
    iid: String,
    users: RwLock<Vec<User>>,
    connections: RwLock<Vec<Connection>>,
    banner: RwLock<Banner>,
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let db = DbConnection::new("http://127.0.0.1:5984", "admin", "password")
        .await
        .unwrap();
    db.setup_db().await;

    // Load saved instance ID
    let iid = db.get_iid().await.unwrap();

    let context = Data::new(ServerContext {
        db: db,
        iid: iid,
        users: RwLock::new(vec![]),
        connections: RwLock::new(vec![]),
    });

    let config = ServerConfig::builder().with_safe_defaults();
    //.with_client_cert_verifier(AllowAnyAuthenticatedClient::new(cert_store));

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
            // Add API endpoints
            .service(api::v1::agent::reboot::reboot)
            .service(api::v1::server::banner::get_banner)
            .service(api::v1::server::session::delete_session)
            .service(api::v1::server::users::add_user)
            .service(api::v1::server::users::list_users)
    })
    .bind_rustls(("localhost", 8443), config)?
    .workers(2)
    .run()
    .await
}
