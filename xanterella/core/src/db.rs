use serde::Serialize;
use sqlx::{sqlite::{SqlitePool, SqlitePoolOptions}, FromRow};

#[derive(Debug, Clone)]
pub struct Database {
    pub pool: SqlitePool,
}

#[derive(Serialize, FromRow)]
pub struct DBHost {
    pub id: i64,
    pub hostname: String,
    pub ip: String,
}

impl Database {
    pub async fn init(db_url: &str) -> Result<Self, sqlx::Error> {
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(db_url)
            .await?;
        sqlx::migrate!("../migrations/").run(&pool).await?;

        Ok(Self { pool})
    }

    pub async fn add_host(&self, hostname: &str, ip: &str) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO hosts (hostname, ip) VALUES (?, ?)",
            hostname, ip
        ).execute(&self.pool).await?;
        Ok(())
    }

    pub async fn list_hosts(&self) -> Result<Vec<DBHost>, sqlx::Error> {
        let hosts = sqlx::query_as!(DBHost, "SELECT id as 'id!', hostname, ip FROM hosts")
            .fetch_all(&self.pool).await?;
        Ok(hosts)
    }

    pub async fn get_host(&self, hostname: &str) -> Result<Option<DBHost>, sqlx::Error> {
        let host = sqlx::query_as!(DBHost, "SELECT id as 'id!', hostname, ip FROM hosts WHERE hostname = ?", hostname)
            .fetch_optional(&self.pool).await?;
        Ok(host)
    }

    pub async fn delete_host(&self, hostname: &str) -> Result<u64, sqlx::Error> {
        let result = sqlx::query!(
            "DELETE FROM hosts WHERE hostname = ?", hostname
        ).execute(&self.pool).await?;
        Ok(result.rows_affected())
    }
}
