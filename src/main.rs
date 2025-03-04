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
mod pages;

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let pool = conn::sqlite_pool();

    let tera_tmpl = embed::load_templates().unwrap();
    let server = HttpServer::new(move || {
        let logger = Logger::default();
        let secret_key = Key::from(
            env::var("SECRET_KEY")
                .expect("SECRET_KEY not found")
                .as_bytes(),
        );

        App::new()
            .wrap(logger)
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                secret_key,
            ))
            .wrap(middleware::check_login::CheckLogin)
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
            .service(pages::login::login_page())
            .service(pages::auth::auth_scope())
            .default_service(web::get().to(pages::error_404::main))
    })
    .bind((
        env::var("IP_BIND_ADDRESS").unwrap_or("127.0.0.1".to_string()),
        8080,
    ))?
    .run();
    server.await
}
