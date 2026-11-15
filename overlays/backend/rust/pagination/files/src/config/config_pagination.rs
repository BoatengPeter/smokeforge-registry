use super::{Config, ConfigError};

pub const MIN_PAGINATION_SECRET: usize = 32;

impl Config {
    pub fn validate_pagination(&self) -> Result<(), ConfigError> {
        if self.pagination_cursor_secret.len() < MIN_PAGINATION_SECRET {
            return Err(ConfigError::Invalid(format!(
                "PAGINATION_CURSOR_SECRET must be at least {MIN_PAGINATION_SECRET} bytes"
            )));
        }

        if self.pagination_default_limit == 0
            || self.pagination_max_limit == 0
            || self.pagination_default_limit > self.pagination_max_limit
        {
            return Err(ConfigError::Invalid(
                "pagination limits are invalid".to_owned(),
            ));
        }

        if self.pagination_max_offset == 0 {
            return Err(ConfigError::Invalid(
                "PAGINATION_MAX_OFFSET must be positive".to_owned(),
            ));
        }

        Ok(())
    }
}
