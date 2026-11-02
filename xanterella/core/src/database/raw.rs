use crate::database::db::{DBRaw, Database};

trait RawString {
    fn into_blop(&self) -> Vec<u8>;
}

impl RawString for Vec<u8> {
    fn into_blop(&self) -> Vec<u8> {
        self.clone()
    }
}

impl RawString for String {
    fn into_blop(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}

impl Database {
    pub async fn add_raw<I: RawString>(
        &self, name: &str, path: &str, content: I, readable: bool,
    ) -> Result<(), sqlx::Error> {
        let blop = content.into_blop();
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
