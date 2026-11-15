use super::{Config, ConfigError};

impl Config {
    pub fn validate_compression(&self) -> Result<(), ConfigError> {
        if self.queue_compression_min_bytes == 0 {
            return Err(ConfigError::Invalid(
                "QUEUE_COMPRESSION_MIN_BYTES must be positive".to_owned(),
            ));
        }

        Ok(())
    }
}
