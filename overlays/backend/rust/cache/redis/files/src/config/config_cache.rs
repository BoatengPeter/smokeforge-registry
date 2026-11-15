use super::{Config, ConfigError};

impl Config {
    pub fn validate_cache(&self) -> Result<(), ConfigError> {
        if !self.redis_url.starts_with("redis://") && !self.redis_url.starts_with("rediss://") {
            return Err(ConfigError::Invalid(
                "REDIS_URL must be a redis:// or rediss:// URL".to_owned(),
            ));
        }

        if self.redis_timeout_seconds == 0 {
            return Err(ConfigError::Invalid(
                "REDIS_TIMEOUT_SECONDS must be positive".to_owned(),
            ));
        }

        Ok(())
    }
}
