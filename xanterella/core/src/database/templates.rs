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

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::database::tests_utils::setup_test_db;

    #[tokio::test]
    async fn db_add_template() {
        let db = setup_test_db().await;
        let value = Vec::new();
        let result = db.add_template("test", "test.test", value).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn db_add_template_and_list_template() {
        let db = setup_test_db().await;
        let value = Vec::new();
        db.add_template("test", "test.test", value.clone()).await.expect("Fehler beim Hinzufügen der Raw-File");

        let templates = db.list_templates().await.expect("Fehler beim Auslesen");
        assert_eq!(templates[0].name, "test");
        assert_eq!(templates[0].path, "test.test");
        assert_eq!(templates[0].config, sqlx::types::Json(Vec::new()));
    }

    #[tokio::test]
    async fn db_add_duplicate_fails() {
        let db = setup_test_db().await;
        let value = Vec::new();
        db.add_template("test", "test.test", value.clone()).await.expect("Fehler beim Hinzufügen der Raw-File");
        let result = db.add_template("test", "test.test.test", value).await;

        assert!(result.is_err(), "Das Hinzufügen eines Duplikates sollte einen Fehler werfen");
    }

    #[tokio::test]
    async fn db_get_template_edge_cases() {
        let db = setup_test_db().await;
        let value = Vec::new();
        db.add_template("test", "test.test", value).await.unwrap();

        let exist = db.get_template("test").await;
        assert!(exist.is_ok());
        assert!(exist.unwrap().is_some());

        let ghost = db.get_template("ghost").await;
        assert!(ghost.is_ok());
        assert!(ghost.unwrap().is_none());
    }

    #[tokio::test]
    async fn db_delete_template_edge_cases() {
        let db = setup_test_db().await;
        let value = Vec::new();
        db.add_template("test", "test.test", value).await.unwrap();

        let affected = db.delete_template("test").await;
        assert!(affected.is_ok());
        assert_eq!(affected.unwrap(), 1);

        let affected_none = db.delete_template("ghost").await;
        assert!(affected_none.is_ok());
        assert_eq!(affected_none.unwrap(), 0);
    }
}
