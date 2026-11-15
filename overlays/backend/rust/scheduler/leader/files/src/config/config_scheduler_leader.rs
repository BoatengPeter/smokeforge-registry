use super::{Config, ConfigError};

impl Config {
    pub fn validate_scheduler_leader(&self) -> Result<(), ConfigError> {
        if self.scheduler_lease_ttl_seconds == 0 {
            return Err(ConfigError::Invalid(
                "SCHEDULER_LEASE_TTL_SECONDS must be positive".to_owned(),
            ));
        }

        if self.scheduler_lease_renew_seconds == 0 {
            return Err(ConfigError::Invalid(
                "SCHEDULER_LEASE_RENEW_SECONDS must be positive".to_owned(),
            ));
        }

        if self.scheduler_lease_renew_seconds >= self.scheduler_lease_ttl_seconds {
            return Err(ConfigError::Invalid(
                "SCHEDULER_LEASE_RENEW_SECONDS must be shorter than SCHEDULER_LEASE_TTL_SECONDS"
                    .to_owned(),
            ));
        }

        if self.scheduler_lease_retry_seconds == 0 {
            return Err(ConfigError::Invalid(
                "SCHEDULER_LEASE_RETRY_SECONDS must be positive".to_owned(),
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_default_lease_settings() {
        assert!(Config::default().validate_scheduler_leader().is_ok());
    }

    #[test]
    fn uses_expected_defaults() {
        let config = Config::default();

        assert_eq!(config.scheduler_lease_ttl_seconds, 30);
        assert_eq!(config.scheduler_lease_renew_seconds, 10);
        assert_eq!(config.scheduler_lease_retry_seconds, 5);
    }

    #[test]
    fn rejects_zero_ttl() {
        let config = Config {
            scheduler_lease_ttl_seconds: 0,
            ..Config::default()
        };

        assert!(config.validate_scheduler_leader().is_err());
    }

    #[test]
    fn rejects_zero_renew() {
        let config = Config {
            scheduler_lease_renew_seconds: 0,
            ..Config::default()
        };

        assert!(config.validate_scheduler_leader().is_err());
    }

    #[test]
    fn rejects_zero_retry() {
        let config = Config {
            scheduler_lease_retry_seconds: 0,
            ..Config::default()
        };

        assert!(config.validate_scheduler_leader().is_err());
    }

    #[test]
    fn rejects_renew_not_shorter_than_ttl() {
        let config = Config {
            scheduler_lease_renew_seconds: 30,
            ..Config::default()
        };

        assert!(config.validate_scheduler_leader().is_err());
    }
}
