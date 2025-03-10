use actix_web::{Scope, web};

mod delete_remove_tab;
mod post_new_tab;

pub fn tabs_scope() -> Scope {
    web::scope("/tabs").service(
        web::resource("")
            .route(web::post().to(post_new_tab::new_tab))
            .route(web::delete().to(delete_remove_tab::remove_tab)),
    )
}
