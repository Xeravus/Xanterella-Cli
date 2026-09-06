pub mod db;
pub mod hosts;
pub mod profiles;
pub mod modules;

use crate::database::db::Database;

#[cfg(test)]
mod tests_utils {
    use super::*;
    use sqlx::sqlite::SqlitePoolOptions;
    use serde_json::json;

    pub async fn setup_test_db() -> Database {
        let pool = SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .expect("Konnte In-Memory-DB nicht erstellen");
        sqlx::migrate!("../migrations").run(&pool).await.expect("Migration fehlgeschlagen");

        Database { pool }
    }
}
