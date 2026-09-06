use sqlx::types::Json;

use crate::database::db::DBHost;
use crate::database::db::Database;

impl Database {
    pub async fn add_host(
        &self, hostname: &str, ip: &str, profiles: Vec<serde_json::Value>, options: Vec<serde_json::Value>,
    ) -> Result<(), sqlx::Error> {
        let json_profiles = Json(profiles);
        let json_options = Json(options);
        sqlx::query!(
            "INSERT INTO hosts (hostname, ip, profiles, options) VALUES (?, ?, ?, ?)",
            hostname,
            ip,
            json_profiles,
            json_options
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn delete_host(&self, hostname: &str) -> Result<u64, sqlx::Error> {
        let result = sqlx::query!("DELETE FROM hosts WHERE hostname = ?", hostname).execute(&self.pool).await?;
        Ok(result.rows_affected())
    }

    pub async fn get_host(&self, hostname: &str) -> Result<Option<DBHost>, sqlx::Error> {
        let host = sqlx::query_as!(DBHost, "SELECT id as 'id!', hostname, ip, profiles as 'profiles: Json<Vec<String>>', options as 'options: Json<Vec<String>>' FROM hosts WHERE hostname = ?", hostname)
            .fetch_optional(&self.pool)
            .await?;
        Ok(host)
    }

    pub async fn list_hosts(&self) -> Result<Vec<DBHost>, sqlx::Error> {
        let hosts = sqlx::query_as!(DBHost, "SELECT id as 'id!', hostname, ip, profiles as 'profiles: Json<Vec<String>>', options as 'options: Json<Vec<String>>' FROM hosts").fetch_all(&self.pool).await?;
        Ok(hosts)
    }

    pub async fn host_add_profile(&self, hostname: &str, profile: serde_json::Value) -> Result<(), sqlx::Error> {
        let mut host = match self.get_host(hostname).await? {
            Some(h) => h,
            None => return Err(sqlx::Error::RowNotFound),
        };
        host.profiles.0.push(profile.to_string());
        let json_host = Json(host.profiles.0);
        sqlx::query!("UPDATE hosts SET profiles = ? WHERE hostname = ?", json_host, hostname)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn host_remove_profile(&self, hostname: &str, profile: serde_json::Value) -> Result<(), sqlx::Error> {
        let mut host = match self.get_host(hostname).await? {
            Some(h) => h,
            None => return Err(sqlx::Error::RowNotFound),
        };
        if let Some(index) = host.profiles.0.iter().position(|e| *e == profile.to_string()) {
            host.profiles.0.swap_remove(index);
        }
        let json_host = Json(host.profiles.0);
        sqlx::query!("UPDATE hosts SET profiles = ? WHERE hostname = ?", json_host, hostname)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn host_add_option(&self, hostname: &str, option: serde_json::Value) -> Result<(), sqlx::Error> {
        let mut host = match self.get_host(hostname).await? {
            Some(h) => h,
            None => return Err(sqlx::Error::RowNotFound),
        };
        host.options.0.push(option.to_string());
        let json_host = Json(host.options.0);
        sqlx::query!("UPDATE hosts SET options = ? WHERE hostname = ?", json_host, hostname)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn host_remove_option(&self, hostname: &str, option: serde_json::Value) -> Result<(), sqlx::Error> {
        let mut host = match self.get_host(hostname).await? {
            Some(h) => h,
            None => return Err(sqlx::Error::RowNotFound),
        };
        if let Some(index) = host.options.0.iter().position(|e| *e == option.to_string()) {
            host.options.0.swap_remove(index);
        }
        let json_host = Json(host.options.0);
        sqlx::query!("UPDATE hosts SET options = ? WHERE hostname = ?", json_host, hostname)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use serde_json::json;
    use sqlx::sqlite::SqlitePoolOptions;

    use super::*;
    use crate::database::tests_utils::setup_test_db;

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
    async fn db_add_duplicate_host_fails() {
        let db = setup_test_db().await;
        db.add_host("dup-node", "1.1.1.1", vec![], vec![]).await.unwrap();
        let result = db.add_host("dup-node", "2.2.2.2", vec![], vec![]).await;
        assert!(result.is_err(), "Das Hinzufügen eines Duplikats sollte einen Fehler werfen");
    }

    #[tokio::test]
    async fn db_get_host_edge_cases() {
        let db = setup_test_db().await;
        db.add_host("test", "1.1.1.1", vec![], vec![]).await.unwrap();
        let exist = db.get_host("test").await.unwrap();
        assert!(exist.is_some());
        let ghost = db.get_host("ghost-node").await.unwrap();
        assert!(ghost.is_none());
    }

    #[tokio::test]
    async fn db_delete_host_edge_cases() {
        let db = setup_test_db().await;
        db.add_host("delete-me", "1.1.1.1", vec![], vec![]).await.unwrap();
        let affected = db.delete_host("delete-me").await.unwrap();
        assert_eq!(affected, 1);
        let affected_none = db.delete_host("ghost-node").await.unwrap();
        assert_eq!(affected_none, 0);
    }

    #[tokio::test]
    async fn db_profiles_crud() {
        let mut db = setup_test_db().await;
        db.add_host("profile-node", "10.0.0.1", vec![], vec![]).await.unwrap();
        db.host_add_profile("profile-node", json!("web-server")).await.unwrap();
        let host = db.get_host("profile-node").await.unwrap().unwrap();
        assert_eq!(host.profiles.0.len(), 1);
        assert_eq!(host.profiles.0[0], json!("web-server").to_string());
        db.host_remove_profile("profile-node", json!("web-server")).await.unwrap();
        let host_after = db.get_host("profile-node").await.unwrap().unwrap();
        assert_eq!(host_after.profiles.0.len(), 0);
        let remove_result = db.host_remove_profile("profile-node", json!("ghost-profile")).await;
        assert!(remove_result.is_ok());
        let err = db.host_add_profile("ghost-node", json!("web-server")).await;
        assert!(matches!(err, Err(sqlx::Error::RowNotFound)));
    }

    #[tokio::test]
    async fn db_options_crud() {
        let mut db = setup_test_db().await;
        db.add_host("options-node", "10.0.0.2", vec![], vec![]).await.unwrap();
        db.host_add_option("options-node", json!("sys.port=80")).await.unwrap();
        let host = db.get_host("options-node").await.unwrap().unwrap();
        assert_eq!(host.options.0.len(), 1);
        assert_eq!(host.options.0[0], json!("sys.port=80").to_string());
        db.host_remove_option("options-node", json!("sys.port=80")).await.unwrap();
        let host_after = db.get_host("options-node").await.unwrap().unwrap();
        assert_eq!(host_after.options.0.len(), 0);
        let err = db.host_remove_option("ghost-node", json!("sys.port=80")).await;
        assert!(matches!(err, Err(sqlx::Error::RowNotFound)));
    }
}
