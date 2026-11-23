use crate::database::db::{DBRaw, Database};

pub trait RawString {
    fn to_blop(&self) -> Vec<u8>;
}

impl RawString for Vec<u8> {
    fn to_blop(&self) -> Vec<u8> {
        self.clone()
    }
}

impl RawString for String {
    fn to_blop(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}

impl Database {
    pub async fn add_raw<I: RawString>(
        &self, name: &str, path: &str, content: I, readable: bool,
    ) -> Result<(), sqlx::Error> {
        let blop = content.to_blop();
        sqlx::query!(
            "INSERT INTO raw (name, path, content, readable) VALUES (?, ?, ?, ?)",
            name,
            path,
            blop,
            readable,
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn delete_raw(&self, name: &str) -> Result<u64, sqlx::Error> {
        let result = sqlx::query!("DELETE FROM raw WHERE name = ?", name).execute(&self.pool).await?;
        Ok(result.rows_affected())
    }

    pub async fn get_raw(&self, name: &str) -> Result<Option<DBRaw>, sqlx::Error> {
        let host = sqlx::query_as!(
            DBRaw,
            "SELECT id as 'id!', name, path, content as 'content: Vec<u8>', readable FROM raw WHERE name = ?",
            name
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(host)
    }

    pub async fn list_raw(&self) -> Result<Vec<DBRaw>, sqlx::Error> {
        let raw =
            sqlx::query_as!(DBRaw, "SELECT id as 'id!', name, path, content as 'content: Vec<u8>', readable FROM raw")
                .fetch_all(&self.pool)
                .await?;
        Ok(raw)
    }
}

#[cfg(test)]
mod tests {
    use crate::database::tests_utils::setup_test_db;

    #[tokio::test]
    async fn db_add_raw() {
        let db = setup_test_db().await;
        let raw = "test".to_string();
        let result = db.add_raw("test", "test.test", raw, true).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn db_add_raw_and_list_raw() {
        let db = setup_test_db().await;
        let raw = "test".to_string();
        db.add_raw("test", "test.test", raw.clone(), true).await.expect("Fehler beim Hinzufügen der Raw-File");

        let raws = db.list_raw().await.expect("Fehler beim Auslesen");
        assert_eq!(raws[0].name, "test");
        assert_eq!(raws[0].path, "test.test");
        assert_eq!(raws[0].content, raw.as_bytes());
        assert_eq!(raws[0].readable, true);
    }
}
