use std::collections::HashMap;
use std::sync::OnceLock;
use std::time::Duration;

use sea_orm::{ConnectOptions, Database, DatabaseConnection};

use crate::config::Config;

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
        let mut options = ConnectOptions::new(
            Config::get::<String>(format!("databases.{}.url", pool).as_str()),
        );

        options
            .connect_timeout(Duration::from_secs(Config::get::<u64>(format!("databases.{}.connection_timeout", pool).as_str())))
            .max_connections(Config::get::<u32>(format!("databases.{}.max_connections", pool).as_str()))
            .min_connections(Config::get::<u32>(format!("databases.{}.min_connections", pool).as_str()))
            .idle_timeout(Duration::from_secs(Config::get::<u64>(format!("databases.{}.idle_timeout", pool).as_str())))
            .acquire_timeout(Duration::from_secs(Config::get::<u64>(format!("databases.{}.acquire_timeout", pool).as_str())))
            .max_lifetime(Duration::from_secs(Config::get::<u64>(format!("databases.{}.max_lifetime", pool).as_str())));

        Database::connect(options).await.unwrap()
    }
}
