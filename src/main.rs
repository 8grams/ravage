extern crate dotenv;
extern crate tera;

use actix_web::{App, HttpServer, middleware::Logger, web};
use app_state::AppState;
use dotenv::dotenv;
use std::env;

mod app_state;
mod pages;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let tera_tmpl = match tera::Tera::new("templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            println!("Error loading templates: {}", e);
            std::process::exit(1);
        }
    };
    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .app_data(web::Data::new(AppState {
                tera: tera_tmpl.clone(),
            }))
            .route("/ping", web::get().to(pages::ping::main))
            .service(pages::login::login_page())
            .default_service(web::get().to(pages::error_404::main))
    })
    .bind((
        env::var("IP_BIND_ADDRESS").unwrap_or("127.0.0.1".to_string()),
        8080,
    ))?
    .run()
    .await
}
