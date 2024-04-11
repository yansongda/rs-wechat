use std::collections::HashMap;
use std::sync::OnceLock;
use std::time::Duration;
use serde::Deserialize;

use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

use crate::config::Config;

pub mod short_url;
pub mod totp;
pub mod user;

pub struct Pool;

static G_POOL_PG: OnceLock<HashMap<&str, PgPool>> = OnceLock::new();

#[derive(Debug, Deserialize)]
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

        for database in databases {
            println!("{:?}", database);
        }
    }

    async fn connect_postgres(config: DatabaseConfig) -> PgPool {
        PgPoolOptions::new()
            .acquire_timeout(Duration::from_secs(config.acquire_timeout))
            .min_connections(config.min_connections)
            .max_connections(config.max_connections)
            .connect(&config.url)
            .await.unwrap()
    }
}
