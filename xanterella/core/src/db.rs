use serde::{Serialize, Deserialize};
use sqlx::{sqlite::{SqlitePool, SqlitePoolOptions}, FromRow, types::Json};

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

#[derive(Serialize, Deserialize, FromRow)]
pub struct DBModul {
    pub id: i64,
    pub name: String,
    pub desc: String,
    pub category: String,
    pub options: sqlx::types::Json<Vec<String>>,
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

    pub async fn add_modul(&self, name: &str, desc: &str, category: &str, options: Vec<serde_json::Value>) -> Result<(), sqlx::Error> {
        let json_value = Json(options);
        sqlx::query!(
            "INSERT INTO modules (name, desc, category, options) VALUES (?, ?, ?, ?)",
            name, desc, category, json_value
        ).execute(&self.pool).await?;
        Ok(())
    }

    pub async fn list_hosts(&self) -> Result<Vec<DBHost>, sqlx::Error> {
        let hosts = sqlx::query_as!(DBHost, "SELECT id as 'id!', hostname, ip FROM hosts")
            .fetch_all(&self.pool).await?;
        Ok(hosts)
    }

    pub async fn list_modules(&self) -> Result<Vec<DBModul>, sqlx::Error> {
        let modules = sqlx::query_as!(DBModul, "SELECT id as 'id!', name, desc, category, options as 'options: Json<Vec<String>>' FROM modules")
            .fetch_all(&self.pool).await?;
        Ok(modules)
    }

    pub async fn get_host(&self, hostname: &str) -> Result<Option<DBHost>, sqlx::Error> {
        let host = sqlx::query_as!(DBHost, "SELECT id as 'id!', hostname, ip FROM hosts WHERE hostname = ?", hostname)
            .fetch_optional(&self.pool).await?;
        Ok(host)
    }

    pub async fn get_modul(&self, name: &str) -> Result<Option<DBModul>, sqlx::Error> {
        let modul = sqlx::query_as!(DBModul, "SELECT id as 'id!', name, desc, category, options as 'options: Json<Vec<String>>' FROM modules WHERE name = ?", name)
            .fetch_optional(&self.pool).await?;
        Ok(modul)
    }

    pub async fn delete_host(&self, hostname: &str) -> Result<u64, sqlx::Error> {
        let result = sqlx::query!(
            "DELETE FROM hosts WHERE hostname = ?", hostname
        ).execute(&self.pool).await?;
        Ok(result.rows_affected())
    }

    pub async fn delete_modul(&self, name: &str) -> Result<u64, sqlx::Error> {
        let result = sqlx::query!(
            "DELETE FROM modules WHERE name = ?", name
        ).execute(&self.pool).await?;
        Ok(result.rows_affected())
    }
}
