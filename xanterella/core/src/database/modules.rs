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
            "SELECT id as 'id!', name, desc, category, options as 'options: Json<Vec<serde_json::Value>>' FROM modules"
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(modules)
    }

    pub async fn list_modules_category(&self, category: &str) -> Result<Vec<DBModul>, sqlx::Error> {
        let modules = sqlx::query_as!(DBModul, "SELECT id as 'id!', name, desc, category, options as 'options: Json<Vec<serde_json::Value>>' FROM modules WHERE category = ?", category).fetch_all(&self.pool).await?;
        Ok(modules)
    }

    pub async fn get_modul(&self, name: &str) -> Result<Option<DBModul>, sqlx::Error> {
        let modul = sqlx::query_as!(DBModul, "SELECT id as 'id!', name, desc, category, options as 'options: Json<Vec<serde_json::Value>>' FROM modules WHERE name = ?", name)
            .fetch_optional(&self.pool).await?;
        Ok(modul)
    }

    pub async fn delete_modul(&self, name: &str) -> Result<u64, sqlx::Error> {
        let result = sqlx::query!("DELETE FROM modules WHERE name = ?", name).execute(&self.pool).await?;
        Ok(result.rows_affected())
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use crate::database::tests_utils::setup_test_db;

    #[tokio::test]
    async fn db_add_modul_and_list_modul() {
        let db = setup_test_db().await;
        let options = vec![json!({"sys.enable": true})];

        db.add_modul("test-modul1", "test modul nr1", "test-cat", options)
            .await
            .expect("Fehler beim Hinzufügen des Modules");

        let modules = db.list_modules().await.expect("Fehler beim Auslesen");
        assert_eq!(modules.len(), 1);
        assert_eq!(modules[0].name, "test-modul1");
        assert_eq!(modules[0].desc, "test modul nr1");
        assert_eq!(modules[0].category, "test-cat");
        assert_eq!(modules[0].options.0[0], json!({"sys.enable": true}));
    }

    #[tokio::test]
    async fn db_get_modul() {
        let db = setup_test_db().await;
        db.add_modul("test-mod", "desc", "cat", vec![]).await.unwrap();

        let get1 = db.get_modul("test-mod").await.unwrap();
        assert!(get1.is_some());

        let get2 = db.get_modul("ghost-mod").await.unwrap();
        assert!(get2.is_none());
    }

    #[tokio::test]
    async fn db_list_modules_category() {
        let db = setup_test_db().await;
        db.add_modul("mod1", "desc", "cat-a", vec![]).await.unwrap();
        db.add_modul("mod2", "desc", "cat-a", vec![]).await.unwrap();
        db.add_modul("mod3", "desc", "cat-b", vec![]).await.unwrap();

        let cat_a = db.list_modules_category("cat-a").await.unwrap();
        assert_eq!(cat_a.len(), 2);

        let cat_c = db.list_modules_category("cat-c").await.unwrap();
        assert_eq!(cat_c.len(), 0);
    }

    #[tokio::test]
    async fn db_delete_modul() {
        let db = setup_test_db().await;
        db.add_modul("test-mod", "desc", "cat", vec![]).await.unwrap();

        let affected_rows = db.delete_modul("test-mod").await.unwrap();
        let modules = db.list_modules().await.unwrap();

        assert_eq!(affected_rows, 1);
        assert_eq!(modules.len(), 0);

        let affected_none = db.delete_modul("ghost-mod").await.unwrap();
        assert_eq!(affected_none, 0);
    }
}
