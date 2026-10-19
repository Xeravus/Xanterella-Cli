#[cfg(test)]
pub mod tests_utils {
    use std::sync::Arc;

    use tokio::sync::broadcast;
    use xanterella_core::{config::AppConfig, database::db::Database};

    use crate::AppState;

    pub async fn test_state() -> Arc<AppState> {
        let (tx, _rx) = broadcast::channel(100);
        let db = Database::init("sqlite::memory:").await.unwrap();
        let config = AppConfig {
            target_directory: String::new(),
            ip: String::new(),
            port: 0,
        };
        Arc::new(AppState {
            tx,
            db,
            config,
        })
    }
}
