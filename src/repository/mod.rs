use std::env;
use std::sync::OnceLock;
use std::time::Duration;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

pub mod user;
pub mod totp;

pub(crate) fn default_pool() -> &'static DatabaseConnection {
    static INSTANCE: OnceLock<DatabaseConnection> = OnceLock::new();

    INSTANCE.get_or_init(async || {
        let options = ConnectOptions::new(env::var("DB_URL").unwrap_or_else(
            |_| "mysql://root:root@127.0.0.1/tools".to_string()).as_str(),
        )
            .connect_timeout(env::var("DB_CONNECT_TIMEOUT").map_or_else(
                |_| Duration::from_secs(1), |v| Duration::from_secs(v.parse().unwrap()),
            ))
            .max_connections(env::var("DB_MAX_CONNECTIONS").map_or_else(
                |_| 30, |v| v.parse().unwrap()),
            )
            .min_connections(env::var("DB_MIN_CONNECTIONS").map_or_else(
                |_| 2, |v| v.parse().unwrap()),
            )
            .idle_timeout(env::var("DB_IDLE_TIMEOUT").map_or_else(
                |_| Duration::from_secs(600), |v| Duration::from_secs(v.parse().unwrap())),
            )
            .acquire_timeout(env::var("DB_ACQUIRE_TIMEOUT").map_or_else(
                |_| Duration::from_secs(3), |v| Duration::from_secs(v.parse().unwrap()),
            ))
            .max_lifetime(env::var("DB_MAX_LIFETIME").map_or_else(
                |_| Duration::from_secs(1800), |v| Duration::from_secs(v.parse().unwrap()),
            ));

        Database::connect(options).await?
    })
}



