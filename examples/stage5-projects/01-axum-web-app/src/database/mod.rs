use sqlx::{PgPool, postgres::PgPoolOptions};
use crate::config::Config;
use crate::error::{AppError, Result};

pub type Database = PgPool;

pub struct DatabaseService {
    pool: Database,
}

impl DatabaseService {
    pub async fn new(config: &Config) -> Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(config.database_url())
            .await?;

        Ok(Self { pool })
    }

    pub fn pool(&self) -> &Database {
        &self.pool
    }

    // Run database migrations
    pub async fn migrate(&self) -> Result<()> {
        sqlx::migrate!("./migrations")
            .run(&self.pool)
            .await
            .map_err(AppError::Database)?;

        Ok(())
    }

    // Test database connection
    pub async fn test_connection(&self) -> Result<()> {
        sqlx::query("SELECT 1")
            .fetch_one(&self.pool)
            .await
            .map_err(AppError::Database)?;

        Ok(())
    }
}

// Re-export for convenience
pub use sqlx;
