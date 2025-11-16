use sqlx::SqlitePool;
use anyhow::Result;
use chrono::Utc;

use super::models::{ClipboardEntry, NewClipboardEntry, ClipboardSearchParams};

pub async fn insert_entry(pool: &SqlitePool, entry: NewClipboardEntry) -> Result<i64> {
    let timestamp = Utc::now().timestamp();

    let result = sqlx::query(
        "INSERT INTO clipboard_entries (content_type, text_content, file_path, metadata, created_at, favorite, source_app)
         VALUES (?, ?, ?, ?, ?, 0, ?)"
    )
    .bind(&entry.content_type)
    .bind(&entry.text_content)
    .bind(&entry.file_path)
    .bind(&entry.metadata)
    .bind(timestamp)
    .bind(&entry.source_app)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}

/// Insert or update clipboard entry. If the same text content already exists,
/// update its timestamp instead of creating a duplicate.
pub async fn upsert_entry(pool: &SqlitePool, entry: NewClipboardEntry) -> Result<i64> {
    let timestamp = Utc::now().timestamp();

    // Check if entry with same text content already exists
    if let Some(ref text_content) = entry.text_content {
        let existing: Option<(i64, bool)> = sqlx::query_as(
            "SELECT id, favorite FROM clipboard_entries WHERE text_content = ? LIMIT 1"
        )
        .bind(text_content)
        .fetch_optional(pool)
        .await?;

        if let Some((id, _favorite)) = existing {
            // Update the timestamp and source_app of existing entry
            sqlx::query(
                "UPDATE clipboard_entries SET created_at = ?, source_app = ? WHERE id = ?"
            )
            .bind(timestamp)
            .bind(&entry.source_app)
            .bind(id)
            .execute(pool)
            .await?;

            return Ok(id);
        }
    }

    // If no existing entry found, insert new one
    insert_entry(pool, entry).await
}

pub async fn get_entry(pool: &SqlitePool, id: i64) -> Result<Option<ClipboardEntry>> {
    let entry = sqlx::query_as::<_, ClipboardEntry>(
        "SELECT * FROM clipboard_entries WHERE id = ?"
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(entry)
}

pub async fn search_entries(pool: &SqlitePool, params: ClipboardSearchParams) -> Result<Vec<ClipboardEntry>> {
    let mut query = String::from(
        "SELECT * FROM clipboard_entries WHERE 1=1"
    );

    // Apply filters
    if let Some(ref search_query) = params.query {
        if !search_query.is_empty() {
            query.push_str(" AND text_content LIKE '%' || ? || '%'");
        }
    }

    if let Some(ref content_type) = params.content_type {
        query.push_str(" AND content_type = ?");
    }

    if params.favorites_only {
        query.push_str(" AND favorite = 1");
    }

    // Sort: favorites first, then by created_at DESC
    query.push_str(" ORDER BY favorite DESC, created_at DESC");

    // Pagination
    query.push_str(" LIMIT ? OFFSET ?");

    let mut sql_query = sqlx::query_as::<_, ClipboardEntry>(&query);

    // Bind parameters
    if let Some(ref search_query) = params.query {
        if !search_query.is_empty() {
            sql_query = sql_query.bind(search_query);
        }
    }

    if let Some(ref content_type) = params.content_type {
        sql_query = sql_query.bind(content_type);
    }

    sql_query = sql_query.bind(params.limit).bind(params.offset);

    let entries = sql_query.fetch_all(pool).await?;
    Ok(entries)
}

pub async fn toggle_favorite(pool: &SqlitePool, id: i64) -> Result<bool> {
    // Get current favorite status
    let current: Option<bool> = sqlx::query_scalar(
        "SELECT favorite FROM clipboard_entries WHERE id = ?"
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    if let Some(is_favorite) = current {
        let new_favorite = !is_favorite;

        sqlx::query("UPDATE clipboard_entries SET favorite = ? WHERE id = ?")
            .bind(new_favorite)
            .bind(id)
            .execute(pool)
            .await?;

        Ok(new_favorite)
    } else {
        Err(anyhow::anyhow!("Entry not found"))
    }
}

pub async fn delete_old_entries(pool: &SqlitePool, days: i64) -> Result<u64> {
    let cutoff_timestamp = Utc::now().timestamp() - (days * 24 * 60 * 60);

    let result = sqlx::query(
        "DELETE FROM clipboard_entries WHERE created_at < ? AND favorite = 0"
    )
    .bind(cutoff_timestamp)
    .execute(pool)
    .await?;

    Ok(result.rows_affected())
}

pub async fn delete_entry(pool: &SqlitePool, id: i64) -> Result<bool> {
    let result = sqlx::query("DELETE FROM clipboard_entries WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(result.rows_affected() > 0)
}

pub async fn get_last_entry(pool: &SqlitePool) -> Result<Option<ClipboardEntry>> {
    let entry = sqlx::query_as::<_, ClipboardEntry>(
        "SELECT * FROM clipboard_entries ORDER BY created_at DESC LIMIT 1"
    )
    .fetch_optional(pool)
    .await?;

    Ok(entry)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::migrations::run_migrations;
    use sqlx::sqlite::SqlitePoolOptions;

    async fn setup_test_db() -> Result<SqlitePool> {
        let pool = SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await?;

        run_migrations(&pool).await?;
        Ok(pool)
    }

    #[tokio::test]
    async fn test_insert_and_retrieve_entry() -> Result<()> {
        let pool = setup_test_db().await?;

        let new_entry = NewClipboardEntry::new_text("Hello, World!".to_string());
        let id = insert_entry(&pool, new_entry).await?;

        assert!(id > 0, "Should return valid ID");

        let retrieved = get_entry(&pool, id).await?;
        assert!(retrieved.is_some(), "Should retrieve entry");

        let entry = retrieved.unwrap();
        assert_eq!(entry.text_content, Some("Hello, World!".to_string()));
        assert_eq!(entry.content_type, "text");
        assert!(!entry.favorite);

        pool.close().await;
        Ok(())
    }

    #[tokio::test]
    async fn test_search_entries() -> Result<()> {
        let pool = setup_test_db().await?;

        // Insert multiple entries
        insert_entry(&pool, NewClipboardEntry::new_text("Hello World".to_string())).await?;
        insert_entry(&pool, NewClipboardEntry::new_text("Rust Programming".to_string())).await?;
        insert_entry(&pool, NewClipboardEntry::new_text("Hello Rust".to_string())).await?;

        // Search for "Hello"
        let params = ClipboardSearchParams {
            query: Some("Hello".to_string()),
            ..Default::default()
        };
        let results = search_entries(&pool, params).await?;

        assert_eq!(results.len(), 2, "Should find 2 entries with 'Hello'");

        // Search for "Rust"
        let params = ClipboardSearchParams {
            query: Some("Rust".to_string()),
            ..Default::default()
        };
        let results = search_entries(&pool, params).await?;

        assert_eq!(results.len(), 2, "Should find 2 entries with 'Rust'");

        pool.close().await;
        Ok(())
    }

    #[tokio::test]
    async fn test_favorite_toggle() -> Result<()> {
        let pool = setup_test_db().await?;

        let id = insert_entry(&pool, NewClipboardEntry::new_text("Test".to_string())).await?;

        // Toggle to favorite
        let is_favorite = toggle_favorite(&pool, id).await?;
        assert!(is_favorite, "Should be favorited");

        // Verify in database
        let entry = get_entry(&pool, id).await?.unwrap();
        assert!(entry.favorite);

        // Toggle back
        let is_favorite = toggle_favorite(&pool, id).await?;
        assert!(!is_favorite, "Should not be favorited");

        pool.close().await;
        Ok(())
    }

    #[tokio::test]
    async fn test_favorites_show_first() -> Result<()> {
        let pool = setup_test_db().await?;

        let id1 = insert_entry(&pool, NewClipboardEntry::new_text("First".to_string())).await?;
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        let id2 = insert_entry(&pool, NewClipboardEntry::new_text("Second".to_string())).await?;

        // Make the first (older) entry a favorite
        toggle_favorite(&pool, id1).await?;

        // Search all
        let results = search_entries(&pool, ClipboardSearchParams::default()).await?;

        assert_eq!(results.len(), 2);
        assert_eq!(results[0].id, id1, "Favorite should be first");
        assert_eq!(results[1].id, id2, "Non-favorite should be second");

        pool.close().await;
        Ok(())
    }

    #[tokio::test]
    async fn test_auto_cleanup_old_entries() -> Result<()> {
        let pool = setup_test_db().await?;

        // Insert entries
        let id1 = insert_entry(&pool, NewClipboardEntry::new_text("Old entry".to_string())).await?;
        let id2 = insert_entry(&pool, NewClipboardEntry::new_text("Recent entry".to_string())).await?;

        // Make id1 old by directly updating created_at
        let old_timestamp = Utc::now().timestamp() - (31 * 24 * 60 * 60); // 31 days ago
        sqlx::query("UPDATE clipboard_entries SET created_at = ? WHERE id = ?")
            .bind(old_timestamp)
            .bind(id1)
            .execute(&pool)
            .await?;

        // Delete entries older than 30 days
        let deleted_count = delete_old_entries(&pool, 30).await?;

        assert_eq!(deleted_count, 1, "Should delete 1 old entry");

        // Verify id1 is deleted and id2 remains
        assert!(get_entry(&pool, id1).await?.is_none());
        assert!(get_entry(&pool, id2).await?.is_some());

        pool.close().await;
        Ok(())
    }

    #[tokio::test]
    async fn test_favorites_not_deleted_by_cleanup() -> Result<()> {
        let pool = setup_test_db().await?;

        let id = insert_entry(&pool, NewClipboardEntry::new_text("Old favorite".to_string())).await?;

        // Make it a favorite
        toggle_favorite(&pool, id).await?;

        // Make it old
        let old_timestamp = Utc::now().timestamp() - (31 * 24 * 60 * 60);
        sqlx::query("UPDATE clipboard_entries SET created_at = ? WHERE id = ?")
            .bind(old_timestamp)
            .bind(id)
            .execute(&pool)
            .await?;

        // Try to delete old entries
        let deleted_count = delete_old_entries(&pool, 30).await?;

        assert_eq!(deleted_count, 0, "Should not delete favorites");
        assert!(get_entry(&pool, id).await?.is_some(), "Favorite should still exist");

        pool.close().await;
        Ok(())
    }

    #[tokio::test]
    async fn test_upsert_creates_new_entry() -> Result<()> {
        let pool = setup_test_db().await?;

        let entry = NewClipboardEntry::new_text("New content".to_string());
        let id = upsert_entry(&pool, entry).await?;

        assert!(id > 0, "Should return valid ID");

        let retrieved = get_entry(&pool, id).await?;
        assert!(retrieved.is_some(), "Should retrieve entry");

        let entry = retrieved.unwrap();
        assert_eq!(entry.text_content, Some("New content".to_string()));

        pool.close().await;
        Ok(())
    }

    #[tokio::test]
    async fn test_upsert_updates_existing_entry() -> Result<()> {
        let pool = setup_test_db().await?;

        // Insert first entry
        let entry1 = NewClipboardEntry::new_text("Duplicate content".to_string());
        let id1 = upsert_entry(&pool, entry1).await?;

        let first_entry = get_entry(&pool, id1).await?.unwrap();
        let first_timestamp = first_entry.created_at;

        // Wait to ensure timestamp difference (timestamps are in seconds)
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

        // Insert duplicate content
        let entry2 = NewClipboardEntry::new_text("Duplicate content".to_string());
        let id2 = upsert_entry(&pool, entry2).await?;

        // Should return same ID
        assert_eq!(id1, id2, "Should return same ID for duplicate content");

        // Verify only one entry exists
        let all_entries = search_entries(&pool, ClipboardSearchParams::default()).await?;
        assert_eq!(all_entries.len(), 1, "Should only have one entry");

        // Verify timestamp was updated
        let updated_entry = get_entry(&pool, id1).await?.unwrap();
        assert!(updated_entry.created_at > first_timestamp, "Timestamp should be updated");

        pool.close().await;
        Ok(())
    }

    #[tokio::test]
    async fn test_upsert_preserves_favorite_status() -> Result<()> {
        let pool = setup_test_db().await?;

        // Insert and favorite an entry
        let entry1 = NewClipboardEntry::new_text("Important content".to_string());
        let id1 = upsert_entry(&pool, entry1).await?;
        toggle_favorite(&pool, id1).await?;

        // Verify it's favorited
        let favorited_entry = get_entry(&pool, id1).await?.unwrap();
        assert!(favorited_entry.favorite, "Entry should be favorited");

        // Insert duplicate content
        let entry2 = NewClipboardEntry::new_text("Important content".to_string());
        let id2 = upsert_entry(&pool, entry2).await?;

        // Should be same ID and still favorited
        assert_eq!(id1, id2, "Should be same entry");
        let still_favorited = get_entry(&pool, id2).await?.unwrap();
        assert!(still_favorited.favorite, "Favorite status should be preserved");

        pool.close().await;
        Ok(())
    }
}
