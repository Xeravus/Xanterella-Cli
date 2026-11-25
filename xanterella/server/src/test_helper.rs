#[cfg(test)]
pub mod tests_utils {
    use std::{path::PathBuf, sync::Arc};

    use tokio::sync::broadcast;
    use xanterella_core::{
        config::{AppConfig, AppPaths},
        database::db::Database,
    };

    use crate::AppState;

    pub async fn test_state() -> Arc<AppState> {
        let (tx, _rx) = broadcast::channel(100);
        let db = Database::init("sqlite::memory:").await.unwrap();
        let config = AppConfig {
            target_directory: String::new(),
            ip: String::new(),
            port: 0,
            repo_url: String::new(),
            repo_main_branch: String::new(),
        };
        let paths = AppPaths {
            config_file: PathBuf::new(),
            database_url: String::new(),
        };
        Arc::new(AppState {
            tx,
            db,
            config,
            paths,
        })
    }
}
