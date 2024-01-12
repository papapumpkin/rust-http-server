use std::env;

pub struct Config {
    pub hostname: String,
    pub port: String,
    pub buffer_size: usize,
}

impl Config {
    pub fn load() -> Result<Self, &'static str> {
        const DEFAULT_HOSTNAME: &str = "127.0.0.1";
        const DEFAULT_PORT: &str = "4221";
        const DEFAULT_BUFFER_SIZE: usize = 1024;

        let hostname = env::var("HOSTNAME").unwrap_or_else(|_| DEFAULT_HOSTNAME.to_string());
        let port = env::var("PORT").unwrap_or_else(|_| DEFAULT_PORT.to_string());
        let buffer_size_str =
            env::var("BUFFER_SIZE").unwrap_or_else(|_| DEFAULT_BUFFER_SIZE.to_string());
        let buffer_size = buffer_size_str
            .parse::<usize>()
            .unwrap_or(DEFAULT_BUFFER_SIZE);

        Ok(Config {
            hostname,
            port,
            buffer_size,
        })
    }
}
