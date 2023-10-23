pub struct WeixinConfig {
    host: String,
    app_id: String,
    app_secret: String,
}

impl WeixinConfig {
    pub fn get_host(&self) -> &str {
        self.host.as_str()
    }

    pub fn get_app_id(&self) -> &str {
        self.app_id.as_str()
    }

    pub fn get_app_secret(&self) -> &str {
        self.app_secret.as_str()
    }
}