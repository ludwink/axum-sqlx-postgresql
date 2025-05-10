use axum::{Router, routing::get};

use crate::db::db_pool;

pub async fn router() -> Router {
    Router::new()
        .route("/", get("Hello word"))
        .with_state(db_pool().await)
}
