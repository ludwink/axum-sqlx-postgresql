use std::{env, time::Duration};

use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

pub async fn db_pool() -> Pool<Postgres> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL undefined");

    PgPoolOptions::new()
        .max_connections(15)
        .acquire_timeout(Duration::from_secs(15))
        .connect(&db_url)
        .await
        .expect("Failed to connect to PostgreSQL")
}
