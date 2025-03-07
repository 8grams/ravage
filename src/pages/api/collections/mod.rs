use actix_web::{Scope, web};

mod get_collections;
mod post_new_collections;

pub fn collections_scope() -> Scope {
    web::scope("/collections")
        .service(
            web::resource("")
                .route(web::post().to(post_new_collections::new_collection))
                .route(web::get().to(get_collections::collections)),
        )
        .service(web::resource("/{id}"))
}
