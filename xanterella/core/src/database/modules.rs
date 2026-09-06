use sqlx::types::Json;

use crate::database::{Database, db::DBModul};

impl Database {
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

    pub async fn list_modules(&self) -> Result<Vec<DBModul>, sqlx::Error> {
        let modules = sqlx::query_as!(
            DBModul,
            "SELECT id as 'id!', name, desc, category, options as 'options: Json<Vec<String>>' FROM modules"
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(modules)
    }

    pub async fn list_modules_category(&self, category: &str) -> Result<Vec<DBModul>, sqlx::Error> {
        let modules = sqlx::query_as!(DBModul, "SELECT id as 'id!', name, desc, category, options as 'options: Json<Vec<String>>' FROM modules WHERE category = ?", category).fetch_all(&self.pool).await?;
        Ok(modules)
    }

    pub async fn get_modul(&self, name: &str) -> Result<Option<DBModul>, sqlx::Error> {
        let modul = sqlx::query_as!(DBModul, "SELECT id as 'id!', name, desc, category, options as 'options: Json<Vec<String>>' FROM modules WHERE name = ?", name)
            .fetch_optional(&self.pool).await?;
        Ok(modul)
    }

    pub async fn delete_modul(&self, name: &str) -> Result<u64, sqlx::Error> {
        let result = sqlx::query!("DELETE FROM modules WHERE name = ?", name).execute(&self.pool).await?;
        Ok(result.rows_affected())
    }
}
