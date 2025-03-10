extern crate dotenvy;
extern crate tera;

use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use actix_web::{App, HttpServer, cookie::Key, middleware::Logger, web};
use app_state::AppState;
use dotenvy::dotenv;
use pages::index;
use std::env;

mod app_state;
pub mod conn;
mod embed;
mod middleware;
pub mod models;
mod pages;
pub mod schema;
pub mod services;
pub mod utils;

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let pool = conn::sqlite_pool();

    let tera_tmpl = embed::load_templates().unwrap();
    let server_address = env::var("IP_BIND_ADDRESS").unwrap_or("127.0.0.1".to_string());
    let server_port = env::var("PORT_BIND_ADDRESS").unwrap_or("8080".to_string());

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let server = HttpServer::new(move || {
        let logger = Logger::new("%a %{User-Agent}i");
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
            }))
            .route(
                "/static/{filename:.*}",
                web::get().to(embed::serve_static_file),
            )
            .route("/ping", web::get().to(pages::ping::main))
            .route("/", web::get().to(index::main_pages))
            .service(pages::request::request_scope())
            .service(pages::login::login_page())
            .service(pages::auth::auth_scope())
            .service(pages::api::api_scope())
            .default_service(web::get().to(pages::error_404::main))
    })
    .bind((server_address.as_str(), server_port.parse::<u16>().unwrap()))?
    .run();
    println!("Please to run pnpm dev, if you run this with bacon");
    println!(
        "Started http server: http://{}:{}",
        server_address, server_port
    );
    server.await
}
