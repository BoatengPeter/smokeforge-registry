use super::{Config, ConfigError};

impl Config {
    pub fn validate_search(&self) -> Result<(), ConfigError> {
        if !self.search_url.starts_with("http://") && !self.search_url.starts_with("https://") {
            return Err(ConfigError::Invalid(
                "SEARCH_URL must be an HTTP(S) URL".to_owned(),
            ));
        }

        if self.search_index.is_empty()
            || !self
                .search_index
                .chars()
                .all(|value| value.is_ascii_alphanumeric() || matches!(value, '_' | '-'))
        {
            return Err(ConfigError::Invalid(
                "SEARCH_INDEX must contain only letters, numbers, underscores, or hyphens"
                    .to_owned(),
            ));
        }

        if self.search_api_key.is_empty() {
            return Err(ConfigError::Invalid(
                "SEARCH_API_KEY must not be empty".to_owned(),
            ));
        }

        if self.search_timeout_seconds == 0 {
            return Err(ConfigError::Invalid(
                "SEARCH_TIMEOUT_SECONDS must be positive".to_owned(),
            ));
        }

        if self.search_max_limit == 0 || self.search_max_limit > 100 {
            return Err(ConfigError::Invalid(
                "SEARCH_MAX_LIMIT must be between 1 and 100".to_owned(),
            ));
        }

        Ok(())
    }
}
