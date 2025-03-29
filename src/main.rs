//! Main application entry point for the Ravage load testing platform.
//! This module initializes the web server, database connection, and sets up all routes.

extern crate dotenvy;
extern crate tera;

use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use actix_web::{App, HttpServer, cookie::Key, middleware::Logger, web};
use app_state::AppState;
use dotenvy::dotenv;
use futures::lock::Mutex;
use pages::index;
use services::websocket::server::LogServer;
use std::{collections::HashMap, env, sync::Arc};

mod app_state;
pub mod conn;
mod embed;
mod middleware;
pub mod models;
mod pages;
pub mod schema;
pub mod services;
pub mod utils;

/// Main entry point for the application.
/// 
/// This function:
/// 1. Initializes environment variables
/// 2. Sets up the database connection pool
/// 3. Loads Tera templates
/// 4. Configures the web server with:
///    - Session management
///    - Authentication middleware
///    - Static file serving
///    - API routes
///    - Error handling
/// 
/// The server can be configured through environment variables:
/// - `IP_BIND_ADDRESS`: Server IP (default: "127.0.0.1")
/// - `PORT_BIND_ADDRESS`: Server port (default: "8080")
/// - `SECRET_KEY`: Session encryption key
/// 
/// Returns a Result indicating success or failure of server startup.
#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let pool = conn::sqlite_pool();

    let tera_tmpl = embed::load_templates().unwrap();
    let server_address = env::var("IP_BIND_ADDRESS").unwrap_or("127.0.0.1".to_string());
    let server_port = env::var("PORT_BIND_ADDRESS").unwrap_or("8080".to_string());
    let log_channels = Arc::new(Mutex::new(HashMap::new()));
    let (server, handler) = LogServer::new();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let server = HttpServer::new(move || {
        let logger = Logger::default();
        let secret_key = Key::from(
            env::var("SECRET_KEY")
                .expect("SECRET_KEY not found")
                .as_bytes(),
        );

        App::new()
            .wrap(logger)
            .wrap(middleware::check_login::CheckLogin)
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                secret_key,
            ))
            .wrap(middleware::refresh::RefreshMiddleware)
            .app_data(web::Data::new(AppState {
                tera: tera_tmpl.clone(),
                pool: pool.clone(),
                log_channels: log_channels.clone(),
                log_server: handler.clone(),
            }))
            .route(
                "/static/{filename:.*}",
                web::get().to(embed::serve_static_file),
            )
            .route("/test", web::get().to(pages::test::test_page))
            .route("/ping", web::get().to(pages::ping::main))
            .route("/", web::get().to(index::main_pages))
            .service(pages::login::login_page())
            .service(pages::collections::collections_scope())
            .service(pages::request::request_scope())
            .service(pages::loadtests::loadtest_scope())
            .service(pages::auth::auth_scope())
            .service(pages::api::api_scope())
            .default_service(web::get().to(pages::error_404::main))
    })
    .bind((server_address.as_str(), server_port.parse::<u16>().unwrap()))?
    .run();
    server.await
}
