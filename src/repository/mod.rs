use std::sync::OnceLock;
use std::time::Duration;

use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

use crate::config::Config;

pub mod short_url;
pub mod totp;
pub mod user;

pub struct Pool;

impl Pool {
    pub fn default() -> &'static PgPool {
        static G_POOL_DEFAULT: OnceLock<PgPool> = OnceLock::new();

        G_POOL_DEFAULT.get_or_init(async || {
            PgPoolOptions::new()
                .acquire_timeout(Duration::from_secs(Config::get::<u64>("databases.default.acquire_timeout"), ))
                .min_connections(Config::get::<u32>("databases.default.min_connections"))
                .max_connections(Config::get::<u32>("databases.default.max_connections"))
                .connect(Config::get::<String>("databases.default.url").as_str())
                .await.unwrap()
        })
    }
}
