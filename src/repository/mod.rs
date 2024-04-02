use std::collections::HashMap;
use std::sync::OnceLock;
use std::time::Duration;
use sqlx::{Database, Postgres};

use crate::config::Config;

pub mod shortlink;
pub mod totp;
pub mod user;

static G_POOL: OnceLock<HashMap<&str, sqlx::Pool<dyn Database>>> = OnceLock::new();

pub struct Pool;

impl Pool {
    pub async fn init() {
        let p = HashMap::from([("default", Self::connect("default").await)]);

        G_POOL.set(p).unwrap();
    }

    pub fn get(pool: &str) -> &sqlx::Pool<dyn Database> {
        G_POOL.get().unwrap().get(pool).unwrap()
    }

    async fn connect(pool: &str) -> DatabaseConnection {
        let url = Config::get::<String>(format!("databases.{}.url", pool).as_str());

        let pool = sqlx::Pool::<Postgres>::connect("postgres://").await?;
        options
            .sqlx_logging(false)
            .connect_timeout(Duration::from_secs(Config::get::<u64>(
                format!("databases.{}.connection_timeout", pool).as_str(),
            )))
            .max_connections(Config::get::<u32>(
                format!("databases.{}.max_connections", pool).as_str(),
            ))
            .min_connections(Config::get::<u32>(
                format!("databases.{}.min_connections", pool).as_str(),
            ))
            .idle_timeout(Duration::from_secs(Config::get::<u64>(
                format!("databases.{}.idle_timeout", pool).as_str(),
            )))
            .acquire_timeout(Duration::from_secs(Config::get::<u64>(
                format!("databases.{}.acquire_timeout", pool).as_str(),
            )))
            .max_lifetime(Duration::from_secs(Config::get::<u64>(
                format!("databases.{}.max_lifetime", pool).as_str(),
            )));

        Database::connect(options).await.unwrap()
    }
}
