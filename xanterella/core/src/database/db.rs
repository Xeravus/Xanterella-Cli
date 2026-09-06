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

#[derive(Serialize, FromRow, Debug, Clone)]
pub struct DBHost {
    pub id: i64,
    pub hostname: String,
    pub ip: String,
    pub profiles: Json<Vec<String>>,
    pub options: Json<Vec<String>>,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct DBModul {
    pub id: i64,
    pub name: String,
    pub desc: String,
    pub category: String,
    pub options: sqlx::types::Json<Vec<String>>,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct DBProfile {
    pub id: i64,
    pub name: String,
    pub dir: String,
    pub options: sqlx::types::Json<Vec<String>>,
}

impl Database {
    pub async fn init(db_url: &str) -> Result<Self, sqlx::Error> {
        let pool = SqlitePoolOptions::new().max_connections(5).connect(db_url).await?;
        sqlx::migrate!("../migrations/").run(&pool).await?;

        Ok(Self {
            pool,
        })
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use sqlx::sqlite::SqlitePoolOptions;

    use super::*;
    use crate::database::tests_utils::setup_test_db;

    #[tokio::test]
    async fn db_add_modul_and_list_modul() {
        let db = setup_test_db().await;
        let options = vec![json!("sys.enable")];
        db.add_modul("test-modul1", "test modul nr1", "test", options)
            .await
            .expect("Fehler beim Hinzufügen des Modules");
        let modules = db.list_modules().await.expect("Fehler beim Auslesen");

        assert_eq!(modules.len(), 1);
        assert_eq!(modules[0].name, "test-modul1");
        assert_eq!(modules[0].desc, "test modul nr1");
        assert_eq!(modules[0].category, "test");
        assert_eq!(modules[0].options.0[0], "sys.enable");
    }

    #[tokio::test]
    async fn db_get_modul() {
        let db = setup_test_db().await;
        db.add_modul("test", "1.1.1.1", "test", vec![]).await.unwrap();
        let get1 = db.get_modul("test");
        let get2 = db.get_modul("test");

        assert!(get1.await.is_ok());
        assert!(get2.await.unwrap().is_some());
    }

    #[tokio::test]
    async fn db_delete_modul() {
        let db = setup_test_db().await;
        db.add_modul("test", "test modul", "test", vec![]).await.unwrap();
        let affected_rows = db.delete_modul("test").await.unwrap();
        let modules = db.list_modules().await.unwrap();
        assert_eq!(affected_rows, 1);
        assert_eq!(modules.len(), 0);
    }
}
