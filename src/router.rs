use axum::{Router, routing::get};

use crate::{authors::authors_routes, db::db_pool};

pub async fn router() -> Router {
    let v1_routes = Router::new().merge(authors_routes());

    Router::new()
        .route("/", get("Hello word"))
        .nest("/v1", v1_routes)
        .with_state(db_pool().await)
}
