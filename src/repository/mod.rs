use std::env;
use std::sync::OnceLock;
use std::time::Duration;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;

pub mod user;
pub mod totp;

pub(crate) fn default() -> &'static SqlitePool {
    static INSTANCE: OnceLock<SqlitePool> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        SqlitePoolOptions::new()
            .max_connections(
                env::var("DB_MAX_CONNECTIONS_DEFAULT").map_or_else(|_| 30, |v| v.parse().unwrap()),
            )
            .min_connections(
                env::var("DB_MIN_CONNECTIONS_DEFAULT").map_or_else(|_| 2, |v| v.parse().unwrap()),
            )
            .acquire_timeout(env::var("DB_ACQUIRE_TIMEOUT_DEFAULT").map_or_else(
                |_| Duration::from_secs(3),
                |v| Duration::from_secs(v.parse().unwrap()),
            ))
            .max_lifetime(env::var("DB_MAX_LIFETIME_DEFAULT").map_or_else(
                |_| Some(Duration::from_secs(1800)),
                |v| Some(Duration::from_secs(v.parse().unwrap())),
            ))
            .idle_timeout(
                env::var("DB_MAX_LIFETIME_DEFAULT")
                    .map_or_else(|_| None, |v| Some(Duration::from_secs(v.parse().unwrap()))),
            )
            .test_before_acquire(true)
            .connect_lazy(
                env::var("DATABASE_URL")
                    .unwrap_or_else(|_| "mysql://root:root@127.0.0.1/tools".to_string())
                    .as_str(),
            )
            .unwrap()
    })
}



