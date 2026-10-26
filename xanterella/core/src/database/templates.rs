use sqlx::types::Json;

use crate::database::db::{DBTemplates, Database};

impl Database {
    pub async fn add_template(
        &self, name: &str, path: &str, config: Vec<serde_json::Value>,
    ) -> Result<(), sqlx::Error> {
        let json_config = Json(config);
        sqlx::query!("INSERT INTO templates (name, path, config) VALUES (?, ?, ?)", name, path, json_config)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn delete_template(&self, name: &str) -> Result<u64, sqlx::Error> {
        let result = sqlx::query!("DELETE FROM templates WHERE name = ?", name).execute(&self.pool).await?;
        Ok(result.rows_affected())
    }

    pub async fn get_template(&self, name: &str) -> Result<Option<DBTemplates>, sqlx::Error> {
        let template = sqlx::query_as!(DBTemplates, "SELECT id as 'id!', name, path, config as 'config: Json<Vec<serde_json::Value>>' FROM templates WHERE name = ?", name).fetch_optional(&self.pool).await?;
        Ok(template)
    }

    pub async fn list_templates(&self) -> Result<Vec<DBTemplates>, sqlx::Error> {
        let templates = sqlx::query_as!(
            DBTemplates,
            "SELECT id as 'id!', name, path, config as 'config: Json<Vec<serde_json::Value>>' FROM templates"
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(templates)
    }
}
