#[cfg(test)]
mod tests {
    use sqlx::sqlite::SqlitePoolOptions;

    #[tokio::test]
    async fn test_sqlite_in_memory() -> Result<(), sqlx::Error> {
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:?cache=shared")
            .await?;

        sqlx::migrate!("./migrations").run(&pool).await?;

        sqlx::query("INSERT INTO songs (artist, title) VALUES (?, ?)")
            .bind("Kenshi Yonezu")
            .bind("Lemon")
            .execute(&pool)
            .await?;

        let row: (String,) = sqlx::query_as("SELECT artist FROM songs WHERE title = ?")
            .bind("Lemon")
            .fetch_one(&pool)
            .await?;

        assert_eq!(row.0, "Kenshi Yonezu");

        Ok(())
    }
}
