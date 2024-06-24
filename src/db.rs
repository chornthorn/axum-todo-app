
use sqlx::sqlite::SqlitePoolOptions;

pub async fn init_db() -> Result<sqlx::SqlitePool, sqlx::Error> {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite:./database.db")
        .await?;

    Ok(pool)
}
