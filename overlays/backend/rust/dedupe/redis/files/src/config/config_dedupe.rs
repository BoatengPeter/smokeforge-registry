use super::{Config, ConfigError};

impl Config {
    pub fn validate_dedupe(&self) -> Result<(), ConfigError> {
        if self.dedupe_ttl_seconds == 0 {
            return Err(ConfigError::Invalid(
                "DEDUPE_TTL_SECONDS must be positive".to_owned(),
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_default_ttl() {
        assert!(Config::default().validate_dedupe().is_ok());
    }

    #[test]
    fn defaults_to_one_day() {
        assert_eq!(Config::default().dedupe_ttl_seconds, 86_400);
    }

    #[test]
    fn rejects_zero_ttl() {
        let config = Config {
            dedupe_ttl_seconds: 0,
            ..Config::default()
        };

        assert!(config.validate_dedupe().is_err());
    }
}
