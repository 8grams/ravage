use actix_web::{Scope, web};

mod delete_user;
mod patch_update_role;
mod post_new_user;

pub fn users_api_scope() -> Scope {
    web::scope("/users")
        .route("", web::post().to(post_new_user::new_user))
        .route("/{}", web::patch().to(patch_update_role::update_role))
        .route("/{id}", web::delete().to(delete_user::delete_user))
}
