use std::sync::OnceLock;
use config::{Config as C, File, FileFormat};
use serde::Deserialize;

static _CONFIG: OnceLock<C> = OnceLock::new();

pub struct Config;

impl Config {
    pub fn init() {
        let config = C::builder()
            .set_default("bin.api.listen", "0.0.0.0").unwrap()
            .set_default("bin.api.port", 8080).unwrap()
            .set_default("databases.default.url", "sqlite://miniprogram.sqlite3").unwrap()
            .set_default("databases.default.connection_timeout", 1).unwrap()
            .set_default("databases.default.idle_timeout", 600).unwrap()
            .set_default("databases.default.max_lifetime", 1800).unwrap()
            .set_default("databases.default.max_connections", 30).unwrap()
            .set_default("databases.default.min_connections", 2).unwrap()
            .set_default("databases.default.acquire_timeout", 3).unwrap()
            .add_source(File::with_name("./config.toml").required(false))
            .build()
            .expect("初始化配置失败");

        _CONFIG.set(config).expect("系统配置初始化失败");
    }

    pub fn get<'de, T: Deserialize<'de>>(key: &str) -> T {
        _CONFIG.get().expect("获取系统配置失败，请检查是否已成功初始化").get(key).unwrap()
    }
}
