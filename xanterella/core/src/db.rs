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

#[derive(Serialize, FromRow)]
#[derive(Debug, Clone)]
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

impl Database {
    pub async fn init(db_url: &str) -> Result<Self, sqlx::Error> {
        let pool = SqlitePoolOptions::new().max_connections(5).connect(db_url).await?;
        sqlx::migrate!("../migrations/").run(&pool).await?;

        Ok(Self {
            pool,
        })
    }

    pub async fn add_host(&self, hostname: &str, ip: &str, profiles: Vec<serde_json::Value>, options: Vec<serde_json::Value>) -> Result<(), sqlx::Error> {
        let json_profiles = Json(profiles);
        let json_options = Json(options);
        sqlx::query!("INSERT INTO hosts (hostname, ip, profiles, options) VALUES (?, ?, ?, ?)", hostname, ip, json_profiles, json_options).execute(&self.pool).await?;
        Ok(())
    }

    pub async fn add_modul(
        &self, name: &str, desc: &str, category: &str, options: Vec<serde_json::Value>,
    ) -> Result<(), sqlx::Error> {
        let json_value = Json(options);
        sqlx::query!(
            "INSERT INTO modules (name, desc, category, options) VALUES (?, ?, ?, ?)",
            name,
            desc,
            category,
            json_value
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn list_hosts(&self) -> Result<Vec<DBHost>, sqlx::Error> {
        let hosts =
            sqlx::query_as!(DBHost, "SELECT id as 'id!', hostname, ip, profiles as 'profiles: Json<Vec<String>>', options as 'options: Json<Vec<String>>' FROM hosts").fetch_all(&self.pool).await?;
        Ok(hosts)
    }

    pub async fn list_modules(&self) -> Result<Vec<DBModul>, sqlx::Error> {
        let modules = sqlx::query_as!(
            DBModul,
            "SELECT id as 'id!', name, desc, category, options as 'options: Json<Vec<String>>' FROM modules"
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(modules)
    }

    pub async fn get_host(&self, hostname: &str) -> Result<Option<DBHost>, sqlx::Error> {
        let host = sqlx::query_as!(DBHost, "SELECT id as 'id!', hostname, ip, profiles as 'profiles: Json<Vec<String>>', options as 'options: Json<Vec<String>>' FROM hosts WHERE hostname = ?", hostname)
            .fetch_optional(&self.pool)
            .await?;
        Ok(host)
    }

    pub async fn get_modul(&self, name: &str) -> Result<Option<DBModul>, sqlx::Error> {
        let modul = sqlx::query_as!(DBModul, "SELECT id as 'id!', name, desc, category, options as 'options: Json<Vec<String>>' FROM modules WHERE name = ?", name)
            .fetch_optional(&self.pool).await?;
        Ok(modul)
    }

    pub async fn delete_host(&self, hostname: &str) -> Result<u64, sqlx::Error> {
        let result = sqlx::query!("DELETE FROM hosts WHERE hostname = ?", hostname).execute(&self.pool).await?;
        Ok(result.rows_affected())
    }

    pub async fn delete_modul(&self, name: &str) -> Result<u64, sqlx::Error> {
        let result = sqlx::query!("DELETE FROM modules WHERE name = ?", name).execute(&self.pool).await?;
        Ok(result.rows_affected())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::sqlite::SqlitePoolOptions;
    use serde_json::json;

    async fn setup_test_db() -> Database {
        let pool = SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .expect("Konnte In-Memory-DB nicht erstellen");
        sqlx::migrate!("../migrations").run(&pool).await.expect("Migration fehlgeschlagen");

        Database { pool }
    }

    #[tokio::test]
    async fn db_add_host_and_list_hosts() {
        let db = setup_test_db().await;
        let profiles = vec![json!("base-profile")];
        let options = vec![json!("sys.enable")];

        db.add_host("test-node-01", "100.100.100.1", profiles, options)
            .await
            .expect("Fehler beim Hinzufügen des Hosts");
        let hosts = db.list_hosts().await.expect("Fehler beim Auslesen");
        
        assert_eq!(hosts.len(), 1);
        assert_eq!(hosts[0].hostname, "test-node-01");
        assert_eq!(hosts[0].ip, "100.100.100.1");
        assert_eq!(hosts[0].profiles.0[0], "base-profile");
    }

    #[tokio::test]
    async fn db_get_host() {
        let db = setup_test_db().await;
        db.add_host("test", "1.1.1.1", vec![], vec![]).await.unwrap();
        let get1 = db.get_host("test");
        let get2 = db.get_host("test");

        assert!(get1.await.is_ok());
        assert!(get2.await.unwrap().is_some());
    }

    #[tokio::test]
    async fn db_delete_host() {
        let db = setup_test_db().await;
        db.add_host("delete-me", "1.1.1.1", vec![], vec![]).await.unwrap();
        let affected_rows = db.delete_host("delete-me").await.unwrap();
        let hosts = db.list_hosts().await.unwrap();
        assert_eq!(affected_rows, 1);
        assert_eq!(hosts.len(), 0);
    }

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
