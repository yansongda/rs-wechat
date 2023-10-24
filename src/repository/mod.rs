use std::collections::HashMap;
use std::env;
use std::sync::OnceLock;
use std::time::Duration;

use sea_orm::{ConnectOptions, Database, DatabaseConnection};

pub mod totp;
pub mod user;

static _POOL: OnceLock<HashMap<&str, DatabaseConnection>> = OnceLock::new();

pub struct Pool;

impl Pool {
    pub async fn init() {
        let p = HashMap::from([("default", Self::connect("default").await)]);

        _POOL.set(p).unwrap();
    }

    pub fn get(pool: &str) -> &DatabaseConnection {
        _POOL.get().unwrap().get(pool).unwrap()
    }

    async fn connect(pool: &str) -> DatabaseConnection {
        let suffix = if "default" != pool { format!("_{}", pool) } else { "".to_string() };

        let mut options = ConnectOptions::new(
            env::var("DB_URL").unwrap_or_else(|_| "mysql://root:root@127.0.0.1/miniprogram".to_string()),
        );

        options
            .connect_timeout(env::var(format!("DB_CONNECT_TIMEOUT{}", suffix)).map_or_else(
                |_| Duration::from_secs(1),
                |v| Duration::from_secs(v.parse().unwrap_or(1)),
            ))
            .max_connections(
                env::var(format!("DB_MAX_CONNECTIONS{}", suffix)).map_or_else(|_| 30, |v| v.parse().unwrap_or(30)),
            )
            .min_connections(
                env::var(format!("DB_MIN_CONNECTIONS{}", suffix)).map_or_else(|_| 2, |v| v.parse().unwrap_or(2)),
            )
            .idle_timeout(env::var(format!("DB_IDLE_TIMEOUT{}", suffix)).map_or_else(
                |_| Duration::from_secs(600),
                |v| Duration::from_secs(v.parse().unwrap_or(600)),
            ))
            .acquire_timeout(env::var(format!("DB_ACQUIRE_TIMEOUT{}", suffix)).map_or_else(
                |_| Duration::from_secs(3),
                |v| Duration::from_secs(v.parse().unwrap_or(3)),
            ))
            .max_lifetime(env::var(format!("DB_MAX_LIFETIME{}", suffix)).map_or_else(
                |_| Duration::from_secs(1800),
                |v| Duration::from_secs(v.parse().unwrap_or(1800)),
            ));

        Database::connect(options).await.unwrap()
    }
}
