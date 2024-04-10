use config::{Config as C, File};
use serde::Deserialize;
use std::sync::OnceLock;

static G_CONFIG: OnceLock<C> = OnceLock::new();

pub struct Config;

impl Config {
    pub fn init() {
        let config = C::builder()
            .set_default("bin.api.listen", "0.0.0.0")
            .unwrap()
            .set_default("bin.api.port", 8080)
            .unwrap()
            .set_default("bin.api.debug", false)
            .unwrap()
            .set_default(
                "databases.default.url",
                "postgres://localhost:5432/miniprogram",
            )
            .unwrap()
            .set_default("databases.default.max_connections", 30)
            .unwrap()
            .set_default("databases.default.min_connections", 1)
            .unwrap()
            .set_default("databases.default.acquire_timeout", 3)
            .unwrap()
            .set_default("wechat.url", "https://api.weixin.qq.com")
            .unwrap()
            .set_default("wechat.app_id", "")
            .unwrap()
            .set_default("wechat.app_secret", "")
            .unwrap()
            .set_default("short_url.domain", "https://u.ysdor.cn")
            .unwrap()
            .add_source(File::with_name("./config.toml").required(false))
            .build()
            .expect("初始化配置失败");

        G_CONFIG.set(config).expect("系统配置初始化失败");
    }

    pub fn get<'de, T: Deserialize<'de>>(key: &str) -> T {
        G_CONFIG
            .get()
            .expect("获取系统配置失败，请检查是否已成功初始化")
            .get(key)
            .unwrap()
    }
}
