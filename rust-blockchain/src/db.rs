use crate::config::AppConfig;
use crate::error::AppResult;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::sync::Arc;
use tracing::info;

pub async fn init_db(config: &AppConfig) -> AppResult<PgPool> {
    info!("Initializing database connection pool");
    
    let pool = PgPoolOptions::new()
        .max_connections(config.database.max_connections)
        .connect(&config.database.url)
        .await?;
    
    // Run migrations if in development mode
    if config.blockchain.network == "devnet" {
        info!("Running database migrations");
        sqlx::migrate!("./migrations")
            .run(&pool)
            .await?;
    }
    
    Ok(pool)
}