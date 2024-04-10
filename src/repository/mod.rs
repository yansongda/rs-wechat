use std::collections::HashMap;
use std::sync::OnceLock;
use std::time::Duration;

use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

use crate::config::Config;

pub mod short_url;
pub mod totp;
pub mod user;

pub struct Pool;

static G_POOL_PG: OnceLock<HashMap<&str, PgPool>> = OnceLock::new();

impl Pool {
    pub async fn init() {
        G_POOL_PG.set()
    }

    pub fn postgres(pool: &str) -> &PgPool {
        G_POOL_PG.get().unwrap().get(pool).unwrap()
    }



    async fn connect_postgres(pool: &str) -> PgPool {
        PgPoolOptions::new()
            .acquire_timeout(Duration::from_secs(Config::get::<u64>(
                "databases.default.acquire_timeout",
            )))
            .min_connections(Config::get::<u32>("databases.default.min_connections"))
            .max_connections(Config::get::<u32>("databases.default.max_connections"))
            .connect(Config::get::<String>("databases.default.url").as_str())
            .await.unwrap()
    }
}
