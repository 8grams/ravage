use actix_web::{Scope, web};

mod delete_user;
mod get_user_modal;
mod post_new_user;
mod post_update_user;

pub fn users_api_scope() -> Scope {
    web::scope("/users")
        .route("", web::post().to(post_new_user::new_user))
        .route("/new", web::get().to(get_user_modal::new_user_modal))
        .route("/{id}", web::post().to(post_update_user::update_user))
        .route("/{id}", web::delete().to(delete_user::delete_user))
        .route("/{id}", web::get().to(get_user_modal::edit_user_modal))
}
