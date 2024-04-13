use serde::Deserialize;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::OnceLock;
use std::time::Duration;

use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::{ConnectOptions, PgPool};
use tracing::log::LevelFilter;

use crate::config::Config;

pub mod short_url;
pub mod totp;
pub mod user;

pub struct Pool;

static G_POOL_PG: OnceLock<HashMap<String, PgPool>> = OnceLock::new();

#[derive(Debug, Clone, Deserialize)]
struct DatabaseConfig {
    url: String,
    max_connections: u32,
    min_connections: u32,
    acquire_timeout: u64,
}

impl Pool {
    /// 注意只能在 APP 启动时调用一次
    pub async fn init() {
        Self::init_databases().await;
    }

    pub fn postgres(pool: &str) -> &PgPool {
        G_POOL_PG.get().unwrap().get(pool).unwrap()
    }

    async fn init_databases() {
        let databases = Config::get::<HashMap<String, DatabaseConfig>>("databases");

        let mut pg: HashMap<String, PgPool> = HashMap::new();

        for database in databases {
            if database.1.url.starts_with("postgres://") {
                pg.insert(database.0, Self::connect_postgres(&database.1).await);
            }
        }

        G_POOL_PG.set(pg).unwrap();
    }

    async fn connect_postgres(config: &DatabaseConfig) -> PgPool {
        let connection_options = PgConnectOptions::from_str(config.url.as_str())
            .unwrap()
            .log_statements(LevelFilter::Debug);

        PgPoolOptions::new()
            .acquire_timeout(Duration::from_secs(config.acquire_timeout))
            .min_connections(config.min_connections)
            .max_connections(config.max_connections)
            .connect_with(connection_options)
            .await
            .unwrap()
    }
}
