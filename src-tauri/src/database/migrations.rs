use sqlx::{SqlitePool, Row};
use anyhow::Result;

pub const INIT_SQL: &str = r#"
CREATE TABLE IF NOT EXISTS clipboard_entries (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    content_type TEXT NOT NULL,
    text_content TEXT,
    file_path TEXT,
    metadata TEXT,
    created_at INTEGER NOT NULL,
    favorite BOOLEAN DEFAULT 0
);

CREATE INDEX IF NOT EXISTS idx_created_at ON clipboard_entries(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_content_type ON clipboard_entries(content_type);
CREATE INDEX IF NOT EXISTS idx_favorite ON clipboard_entries(favorite);

CREATE TABLE IF NOT EXISTS schema_version (
    version INTEGER PRIMARY KEY,
    applied_at INTEGER NOT NULL
);
"#;

pub async fn run_migrations(pool: &SqlitePool) -> Result<()> {
    // Create tables
    sqlx::query(INIT_SQL).execute(pool).await?;

    // Check current schema version
    let version: Option<i64> = sqlx::query("SELECT version FROM schema_version ORDER BY version DESC LIMIT 1")
        .fetch_optional(pool)
        .await?
        .and_then(|row| row.try_get("version").ok());

    let current_version = version.unwrap_or(0);

    if current_version == 0 {
        // First migration
        let timestamp = chrono::Utc::now().timestamp();
        sqlx::query("INSERT INTO schema_version (version, applied_at) VALUES (?, ?)")
            .bind(1)
            .bind(timestamp)
            .execute(pool)
            .await?;
    }

    // Migration 2: Add source_app column
    if current_version < 2 {
        sqlx::query("ALTER TABLE clipboard_entries ADD COLUMN source_app TEXT")
            .execute(pool)
            .await?;

        let timestamp = chrono::Utc::now().timestamp();
        sqlx::query("INSERT INTO schema_version (version, applied_at) VALUES (?, ?)")
            .bind(2)
            .bind(timestamp)
            .execute(pool)
            .await?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::sqlite::SqlitePoolOptions;

    #[tokio::test]
    async fn test_migrations() -> Result<()> {
        // Create in-memory database
        let pool = SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await?;

        // Run migrations
        run_migrations(&pool).await?;

        // Verify tables exist
        let table_count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name IN ('clipboard_entries', 'schema_version')"
        )
        .fetch_one(&pool)
        .await?;

        assert_eq!(table_count, 2, "Should have 2 tables");

        // Verify schema version
        let version: i64 = sqlx::query_scalar(
            "SELECT version FROM schema_version ORDER BY version DESC LIMIT 1"
        )
        .fetch_one(&pool)
        .await?;

        assert_eq!(version, 1, "Schema version should be 1");

        pool.close().await;
        Ok(())
    }
}
