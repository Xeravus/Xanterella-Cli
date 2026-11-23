use std::collections::HashMap;
use crate::Events;

use serde::{Deserialize, Serialize};
use sqlx::{
    FromRow,
    sqlite::{SqlitePool, SqlitePoolOptions},
    types::Json,
};

#[derive(Debug, Clone)]
pub struct Database {
    pub pool: SqlitePool,
}

#[derive(Serialize, Deserialize, FromRow, Debug, Clone)]
pub struct DBHost {
    pub id: i64,
    pub hostname: String,
    pub ip: String,
    pub system: String,
    pub remote_deploy: bool,
    pub profiles: Json<Vec<serde_json::Value>>,
    pub hardware: Json<Vec<serde_json::Value>>,
    pub options: Json<Vec<serde_json::Value>>,
    pub config: Json<Vec<serde_json::Value>>,
    pub imports: Json<Vec<serde_json::Value>>,
}

#[derive(Serialize, Deserialize, FromRow, Debug, Clone)]
pub struct DBModul {
    pub id: i64,
    pub name: String,
    pub desc: String,
    pub category: String,
    pub options: Json<Vec<serde_json::Value>>,
    pub config: Json<Vec<serde_json::Value>>,
}

#[derive(Serialize, Deserialize, FromRow, Debug, Clone)]
pub struct DBProfile {
    pub id: i64,
    pub name: String,
    pub dir: String,
    pub options: Json<Vec<serde_json::Value>>,
}

#[derive(Serialize, Deserialize, FromRow, Debug, Clone)]
pub struct DBTemplates {
    pub id: i64,
    pub name: String,
    pub path: String,
    pub config: Json<Vec<serde_json::Value>>,
}

#[derive(Serialize, Deserialize, FromRow, Debug, Clone)]
pub struct DBRaw {
    pub id: i64,
    pub name: String,
    pub path: String,
    pub content: Vec<u8>,
    pub readable: bool,
}

impl Database {
    pub async fn init(db_url: &str) -> Result<Self, sqlx::Error> {
        let pool = SqlitePoolOptions::new().max_connections(5).connect(db_url).await?;
        sqlx::migrate!("../migrations/").run(&pool).await?;

        Ok(Self {
            pool,
        })
    }

    pub async fn clear(&self) -> Result<HashMap<String, u64>, sqlx::Error> {
        let mut result = HashMap::new();
        if let Ok(hosts) = sqlx::query!("DELETE FROM hosts").execute(&self.pool).await {
            result.insert("Hosts".to_string(), hosts.rows_affected());
        }
        if let Ok(profiles) = sqlx::query!("DELETE FROM profiles").execute(&self.pool).await {
            result.insert("Profiles".to_string(), profiles.rows_affected());
        }
        if let Ok(modules) = sqlx::query!("DELETE FROM modules").execute(&self.pool).await {
            result.insert("Modules".to_string(), modules.rows_affected());
        }
        if let Ok(templates) = sqlx::query!("DELETE FROM templates").execute(&self.pool).await {
            result.insert("Templates".to_string(), templates.rows_affected());
        }
        if let Ok(raw) = sqlx::query!("DELETE FROM raw").execute(&self.pool).await {
            result.insert("Raw".to_string(), raw.rows_affected());
        }
        Ok(result)
    }
}
