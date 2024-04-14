use config::{Config as C, Environment, File};
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::OnceLock;

static G_CONFIG: OnceLock<Config> = OnceLock::new();

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub name: String,
    pub bin: HashMap<String, Bin>,
    pub databases: HashMap<String, Database>,
    pub wechat: Wechat,
    pub short_url: ShortUrl,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Bin {
    pub listen: String,
    pub port: u16,
    pub debug: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Database {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub acquire_timeout: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Wechat {
    pub url: String,
    pub app_id: String,
    pub app_secret: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ShortUrl {
    pub domain: String,
}

impl Config {
    pub fn init() {
        if G_CONFIG.get().is_some() {
            return;
        }

        let config = C::builder()
            .add_source(File::with_name("./config.toml").required(false))
            .add_source(
                Environment::with_prefix("APP")
                    .try_parsing(true)
                    .separator("_"),
            )
            .build()
            .expect("加载配置失败");

        let instance = config.try_deserialize::<Config>().expect("解析配置失败");

        G_CONFIG.set(instance).expect("系统配置初始化失败");
    }

    pub fn get_name() -> &'static str {
        G_CONFIG.get().unwrap().name.as_str()
    }

    pub fn get_bin(name: &str) -> &Bin {
        G_CONFIG.get().unwrap().bin.get(name).unwrap()
    }

    pub fn get_databases() -> &'static HashMap<String, Database> {
        &G_CONFIG.get().unwrap().databases
    }

    pub fn get_wechat() -> &'static Wechat {
        &G_CONFIG.get().unwrap().wechat
    }

    pub fn get_short_url() -> &'static ShortUrl {
        &G_CONFIG.get().unwrap().short_url
    }
}
