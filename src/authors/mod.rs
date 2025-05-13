use axum::{
    Router,
    routing::{delete, get, patch, post},
};
use sqlx::PgPool;

mod dto;
mod entity;
mod service;

pub fn authors_routes() -> Router<PgPool> {
    let public_routes = Router::new()
        .route("/", get(service::get_all))
        .route("/{id}", get(service::get_by_id));

    let private_routes = Router::new()
        .route("/", post(service::create))
        .route("/{id}", patch(service::patch_update))
        .route("/{id}", delete(service::delete));

    let merged_routes = Router::new().merge(public_routes).merge(private_routes);

    Router::new().nest("/authors", merged_routes)
}
