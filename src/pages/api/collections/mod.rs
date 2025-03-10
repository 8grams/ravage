use actix_web::{Scope, web};

mod delete_collection;
mod get_children_list;
mod get_collection_requests;
mod get_collections;
mod get_edit_collection_modal_form;
mod get_new_collection_modal_form;
mod post_new_collection;
mod post_update_collection;

pub fn collections_scope() -> Scope {
    web::scope("/collections")
        .service(
            web::resource("")
                .route(web::post().to(post_new_collection::new_collection))
                .route(web::get().to(get_collections::collections)),
        )
        .route(
            "/new",
            web::get().to(get_new_collection_modal_form::new_collection_modal_form),
        )
        .service(
            web::resource("/{id}")
                .route(web::post().to(post_update_collection::update_collection))
                .route(web::delete().to(delete_collection::remove_single_collection)),
        )
        .route(
            "/{id}/childrens",
            web::get().to(get_children_list::children_list),
        )
        .route(
            "/{id}/edit",
            web::get().to(get_edit_collection_modal_form::edit_collection_modal_form),
        )
}
