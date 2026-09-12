pub mod db;
pub mod hosts;
pub mod modules;
pub mod profiles;

use crate::database::db::Database;

#[cfg(test)]
mod tests_utils {
    
    use sqlx::sqlite::SqlitePoolOptions;

    use super::*;

    pub async fn setup_test_db() -> Database {
        let pool =
            SqlitePoolOptions::new().connect("sqlite::memory:").await.expect("Konnte In-Memory-DB nicht erstellen");
        sqlx::migrate!("../migrations").run(&pool).await.expect("Migration fehlgeschlagen");

        Database {
            pool,
        }
    }
}
