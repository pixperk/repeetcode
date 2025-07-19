use std::env;

use sqlx::{PgPool, postgres::PgPoolOptions};

pub async fn init_db() -> PgPool {
    dotenv::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to connect to DB")
}
