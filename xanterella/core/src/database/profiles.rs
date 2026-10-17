use sqlx::types::Json;

use crate::database::{
    Database,
    db::{DBHost, DBProfile},
};

impl Database {
    pub async fn add_profile(&self, name: &str, dir: &str, options: Vec<serde_json::Value>) -> Result<(), sqlx::Error> {
        let json_options = Json(options);
        sqlx::query!("INSERT INTO profiles (name, dir, options) VALUES (?, ?, ?)", name, dir, json_options)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn list_profiles(&self) -> Result<Vec<DBProfile>, sqlx::Error> {
        let profiles = sqlx::query_as!(
            DBProfile,
            "SELECT id as 'id!', name, dir, options as 'options: Json<Vec<serde_json::Value>>' FROM profiles"
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(profiles)
    }

    pub async fn get_profile(&self, name: &str) -> Result<Option<DBProfile>, sqlx::Error> {
        let profile = sqlx::query_as!(
            DBProfile,
            "SELECT id as 'id!', name, dir, options as 'options: Json<Vec<serde_json::Value>>' FROM profiles WHERE name = ?",
            name
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(profile)
    }

    pub async fn delete_profile(&self, name: &str) -> Result<u64, sqlx::Error> {
        let result = sqlx::query!("DELETE FROM profiles WHERE name = ?", name).execute(&self.pool).await?;
        Ok(result.rows_affected())
    }

    pub async fn profile_get_hosts(&self, profile_name: &str) -> Result<Vec<DBHost>, sqlx::Error> {
        let hosts = sqlx::query_as!(
            DBHost,
            r#"
            SELECT 
                id as "id!", 
                hostname, 
                ip, 
                profiles as "profiles: Json<Vec<serde_json::Value>>", 
                hardware as "hardware: Json<Vec<serde_json::Value>>", 
                options as "options: Json<Vec<serde_json::Value>>" 
            FROM hosts 
            WHERE EXISTS (
                SELECT 1 FROM json_each(profiles) WHERE value = ?
            )
            "#,
            profile_name
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(hosts)
    }

    pub async fn profile_add_option(&self, profile_name: &str, option: serde_json::Value) -> Result<(), sqlx::Error> {
        let mut profile = match self.get_profile(profile_name).await? {
            Some(p) => p,
            None => return Err(sqlx::Error::RowNotFound),
        };
        profile.options.0.push(option);
        let json_profile = Json(profile.options.0);
        sqlx::query!("UPDATE profiles SET options = ? WHERE name = ?", json_profile, profile_name)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn profile_remove_option(
        &self, profile_name: &str, option: serde_json::Value,
    ) -> Result<(), sqlx::Error> {
        let mut profile = match self.get_profile(profile_name).await? {
            Some(p) => p,
            None => return Err(sqlx::Error::RowNotFound),
        };
        if let Some(index) = profile.options.0.iter().position(|e| *e == option) {
            profile.options.0.swap_remove(index);
        }
        let json_profile = Json(profile.options.0);
        sqlx::query!("UPDATE profiles SET options = ? WHERE name = ?", json_profile, profile_name)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::database::tests_utils::setup_test_db;

    #[tokio::test]
    async fn db_add_profile_and_list_profiles() {
        let db = setup_test_db().await;
        let options = vec![json!({"sys.enable": true})];

        db.add_profile("web-server", "profiles/server", options).await.expect("Fehler beim Hinzufügen des Profils");

        let profiles = db.list_profiles().await.expect("Fehler beim Auslesen");
        assert_eq!(profiles.len(), 1);
        assert_eq!(profiles[0].name, "web-server");
        assert_eq!(profiles[0].dir, "profiles/server");
        assert_eq!(profiles[0].options.0[0], json!({"sys.enable": true}));
    }

    #[tokio::test]
    async fn db_get_profile_edge_cases() {
        let db = setup_test_db().await;
        db.add_profile("test-profile", "dir", vec![]).await.unwrap();

        let exist = db.get_profile("test-profile").await.unwrap();
        assert!(exist.is_some());

        let ghost = db.get_profile("ghost-profile").await.unwrap();
        assert!(ghost.is_none());
    }

    #[tokio::test]
    async fn db_delete_profile_edge_cases() {
        let db = setup_test_db().await;
        db.add_profile("delete-me", "dir", vec![]).await.unwrap();

        let affected = db.delete_profile("delete-me").await.unwrap();
        assert_eq!(affected, 1);

        let affected_none = db.delete_profile("ghost-profile").await.unwrap();
        assert_eq!(affected_none, 0);
    }

    #[tokio::test]
    async fn db_profile_options_crud() {
        let db = setup_test_db().await;
        db.add_profile("options-profile", "dir", vec![]).await.unwrap();

        db.profile_add_option("options-profile", json!({"port": 8080})).await.unwrap();
        let profile = db.get_profile("options-profile").await.unwrap().unwrap();
        assert_eq!(profile.options.0.len(), 1);
        assert_eq!(profile.options.0[0], json!({"port": 8080}));

        db.profile_remove_option("options-profile", json!({"port": 8080})).await.unwrap();
        let profile_after = db.get_profile("options-profile").await.unwrap().unwrap();
        assert_eq!(profile_after.options.0.len(), 0);

        let err_add = db.profile_add_option("ghost-profile", json!({"port": 8080})).await;
        assert!(matches!(err_add, Err(sqlx::Error::RowNotFound)));

        let err_remove = db.profile_remove_option("ghost-profile", json!({"port": 8080})).await;
        assert!(matches!(err_remove, Err(sqlx::Error::RowNotFound)));
    }

    #[tokio::test]
    async fn db_profile_get_hosts() {
        let db = setup_test_db().await;

        let profiles = vec![json!("target-profile"), json!("other-profile")];
        db.add_host("test-node", "1.1.1.1", profiles, vec![], vec![]).await.unwrap();
        db.add_host("other-node", "2.2.2.2", vec![json!("other-profile")], vec![], vec![]).await.unwrap();

        let hosts = db.profile_get_hosts("target-profile").await.unwrap();
        assert_eq!(hosts.len(), 1);
        assert_eq!(hosts[0].hostname, "test-node");

        let empty_hosts = db.profile_get_hosts("\"ghost-profile\"").await.unwrap();
        assert_eq!(empty_hosts.len(), 0);
    }
}
