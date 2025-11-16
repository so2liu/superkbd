pub mod models;
pub mod migrations;
pub mod queries;

use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::SqlitePool;
use std::path::PathBuf;
use std::str::FromStr;
use anyhow::Result;

pub use models::*;
pub use queries::*;

pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn new(db_path: PathBuf) -> Result<Self> {
        // Ensure parent directory exists
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let connection_options = SqliteConnectOptions::from_str(&format!("sqlite://{}", db_path.display()))?
            .create_if_missing(true);

        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect_with(connection_options)
            .await?;

        // Run migrations
        migrations::run_migrations(&pool).await?;

        Ok(Self { pool })
    }

    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_database_creation() -> Result<()> {
        let temp_dir = tempdir()?;
        let db_path = temp_dir.path().join("test.db");

        let db = Database::new(db_path.clone()).await?;

        // Verify database file was created
        assert!(db_path.exists(), "Database file should exist");

        // Verify we can insert and retrieve
        let entry = NewClipboardEntry::new_text("Test entry".to_string());
        let id = queries::insert_entry(db.pool(), entry).await?;

        let retrieved = queries::get_entry(db.pool(), id).await?;
        assert!(retrieved.is_some());

        Ok(())
    }
}
