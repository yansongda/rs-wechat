use std::collections::HashMap;
use std::env;
use std::sync::OnceLock;
use std::time::Duration;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

pub mod user;
pub mod totp;

static POOL: OnceLock<HashMap<&str, DatabaseConnection>> = OnceLock::new();

pub struct Pool;

impl Pool {
    pub async fn init() {
        let p = HashMap::from([
            ("default", Self::default().await),
        ]);

        POOL.set(p).unwrap();
    }

    pub fn get(pool: &str) -> &DatabaseConnection {
        POOL.get().unwrap().get(pool).unwrap()
    }

    async fn default() -> DatabaseConnection {
        let mut options = ConnectOptions::new(env::var("DB_URL").unwrap_or_else(
            |_| "mysql://root:root@127.0.0.1/tools".to_string(),
        ));

        options
            .connect_timeout(env::var("DB_CONNECT_TIMEOUT").map_or_else(
                |_| Duration::from_secs(1), |v| Duration::from_secs(v.parse().unwrap_or(1)),
            ))
            .max_connections(env::var("DB_MAX_CONNECTIONS").map_or_else(
                |_| 30, |v| v.parse().unwrap_or(30)),
            )
            .min_connections(env::var("DB_MIN_CONNECTIONS").map_or_else(
                |_| 2, |v| v.parse().unwrap_or(2)),
            )
            .idle_timeout(env::var("DB_IDLE_TIMEOUT").map_or_else(
                |_| Duration::from_secs(600), |v| Duration::from_secs(v.parse().unwrap_or(600))),
            )
            .acquire_timeout(env::var("DB_ACQUIRE_TIMEOUT").map_or_else(
                |_| Duration::from_secs(3), |v| Duration::from_secs(v.parse().unwrap_or(3)),
            ))
            .max_lifetime(env::var("DB_MAX_LIFETIME").map_or_else(
                |_| Duration::from_secs(1800), |v| Duration::from_secs(v.parse().unwrap_or(1800)),
            ));

        Database::connect(options).await.unwrap()
    }
}
