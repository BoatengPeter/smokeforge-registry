use super::{Config, ConfigError};

impl Config {
    pub fn validate_queue(&self) -> Result<(), ConfigError> {
        if !self.rabbitmq_url.starts_with("amqp://") && !self.rabbitmq_url.starts_with("amqps://") {
            return Err(ConfigError::Invalid(
                "RABBITMQ_URL must be an amqp:// or amqps:// URL".to_owned(),
            ));
        }

        if self.rabbitmq_queue.is_empty() || self.rabbitmq_dlq.is_empty() {
            return Err(ConfigError::Invalid(
                "RabbitMQ queue names must not be empty".to_owned(),
            ));
        }

        if self.rabbitmq_prefetch == 0 || self.scheduler_interval_seconds == 0 {
            return Err(ConfigError::Invalid(
                "RabbitMQ prefetch and scheduler interval must be positive".to_owned(),
            ));
        }

        Ok(())
    }
}
