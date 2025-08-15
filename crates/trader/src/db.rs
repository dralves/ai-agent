use anyhow::{anyhow, Result};
use tracing::debug;

#[allow(dead_code)]
pub enum DatabasePool {
    Sqlite(sqlx::SqlitePool),
    Postgres(sqlx::PgPool),
}

pub async fn connect(database_url: &str) -> Result<DatabasePool> {
    if database_url.starts_with("postgres://") || database_url.starts_with("postgresql://") {
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;
        debug!("connected to postgres");
        Ok(DatabasePool::Postgres(pool))
    } else if database_url.starts_with("sqlite:") {
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;
        debug!("connected to sqlite");
        Ok(DatabasePool::Sqlite(pool))
    } else {
        Err(anyhow!("unsupported database url: {}", database_url))
    }
}


