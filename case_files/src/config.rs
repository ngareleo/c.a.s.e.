pub struct Config {
    pub tg_api_key: String,
    pub port: u16,
}

impl Config {
    pub fn build(api_key: String, port: u16) -> Self {
        return Config {
            tg_api_key: api_key,
            port,
        };
    }
}
