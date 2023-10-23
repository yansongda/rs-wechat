mod api;
mod databases;
mod weixin;

use std::collections::HashMap;

pub struct Config {
    api: ApiConfig,
    databases: HashMap<String, DatabaseConfig>,
    weixin: WeixinConfig,
}

impl Config {


    pub fn get_api(&self) -> &ApiConfig {
        &self.api
    }

    pub fn get_databases(&self) -> &HashMap<String, DatabaseConfig> {
        &self.databases
    }

    pub fn get_weixin(&self) -> &WeixinConfig {
        &self.weixin
    }
}
