pub struct ApiConfig {
    listen: String,
    port: u16,
}

impl ApiConfig {
    pub fn get_listen(&self) -> &str {
        self.listen.as_str()
    }

    pub fn get_port(&self) -> u16 {
        self.port
    }
}