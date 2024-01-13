use std::env;

pub struct Settings {
    pub hostname: String,
    pub port: String,
    pub buffer_size: usize,
}

impl Settings {
    pub async fn load() -> Result<Self, &'static str> {
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

        Ok(Settings {
            hostname,
            port,
            buffer_size,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_load_settings() {
        let settings: Settings = Settings::load().await.expect("Failed to load settings");

        // test that default values are loaded if nothing is specified
        assert_eq!(settings.buffer_size, 1024);
        assert_eq!(settings.hostname, "127.0.0.1");
        assert_eq!(settings.port, "4221");
    }

    #[tokio::test]
    async fn test_load_settings_from_env() {
        // setting env vars, these should be loaded by settings
        env::set_var("HOSTNAME", "dummy_host");
        env::set_var("PORT", "1");
        env::set_var("BUFFER_SIZE", "512");

        let settings: Settings = Settings::load().await.expect("Failed to load settings");

        assert_eq!(
            settings.buffer_size,
            env::var("BUFFER_SIZE")
                .unwrap()
                .parse()
                .expect("BUFFER_SIZE must be a number")
        );
        assert_eq!(settings.hostname, env::var("HOSTNAME").unwrap());
        assert_eq!(settings.port, env::var("PORT").unwrap());

        // cleanup after tests
        std::env::remove_var("HOSTNAME");
        std::env::remove_var("PORT");
        std::env::remove_var("BUFFER_SIZE");
    }
}
