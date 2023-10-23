pub struct DatabaseConfig {
    url: String,
    options_connect_timeout: u64,
    options_max_connections: u32,
    options_min_connections: u32,
    options_idle_timeout: u64,
    options_acquire_timeout: u64,
    options_max_lifetime: u64,
}

impl DatabaseConfig {
    pub fn get_url(&self) -> &str {
        self.url.as_str()
    }

    pub fn get_options_connect_timeout(&self) -> u64 {
        self.options_connect_timeout
    }

    pub fn get_options_max_connections(&self) -> u32 {
        self.options_max_connections
    }

    pub fn get_options_min_connections(&self) -> u32 {
        self.options_min_connections
    }

    pub fn get_options_idle_timeout(&self) -> u64 {
        self.options_idle_timeout
    }

    pub fn get_options_acquire_timeout(&self) -> u64 {
        self.options_acquire_timeout
    }

    pub fn get_options_max_lifetime(&self) -> u64 {
        self.options_max_lifetime
    }
}