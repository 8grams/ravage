use actix_web::{Scope, web};

mod delete_remove_request;
mod post_new_request;
mod post_run;
mod post_update_request;

pub fn requests_scope() -> Scope {
    web::scope("/requests")
        .service(web::resource("").route(web::post().to(post_new_request::new_request)))
        .service(
            web::resource("/{id}")
                .route(web::post().to(post_update_request::update_request))
                .route(web::delete().to(delete_remove_request::remove_request)),
        )
        .route("/{id}/run", web::post().to(post_run::run))
}
