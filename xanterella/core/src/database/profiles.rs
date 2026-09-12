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
            "SELECT id as 'id!', name, dir, options as 'options: Json<Vec<String>>' FROM profiles"
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(profiles)
    }

    pub async fn get_profile(&self, name: &str) -> Result<Option<DBProfile>, sqlx::Error> {
        let profile = sqlx::query_as!(
            DBProfile,
            "SELECT id as 'id!', name, dir, options as 'options: Json<Vec<String>>' FROM profiles WHERE name = ?",
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
                profiles as "profiles: Json<Vec<String>>", 
                options as "options: Json<Vec<String>>" 
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
        profile.options.0.push(option.to_string());
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
        if let Some(index) = profile.options.0.iter().position(|e| *e == option.to_string()) {
            profile.options.0.swap_remove(index);
        }
        let json_profile = Json(profile.options.0);
        sqlx::query!("UPDATE profiles SET options = ? WHERE name = ?", json_profile, profile_name)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
