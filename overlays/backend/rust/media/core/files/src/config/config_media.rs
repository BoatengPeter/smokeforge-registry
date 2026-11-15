use super::{Config, ConfigError};

impl Config {
    pub fn validate_media(&self) -> Result<(), ConfigError> {
        if !self.media_s3_endpoint.starts_with("http://")
            && !self.media_s3_endpoint.starts_with("https://")
        {
            return Err(ConfigError::Invalid(
                "MEDIA_S3_ENDPOINT must be an HTTP(S) URL".to_owned(),
            ));
        }

        if !self.media_s3_public_endpoint.starts_with("http://")
            && !self.media_s3_public_endpoint.starts_with("https://")
        {
            return Err(ConfigError::Invalid(
                "MEDIA_S3_PUBLIC_ENDPOINT must be an HTTP(S) URL".to_owned(),
            ));
        }

        if self.media_s3_bucket.is_empty()
            || self.media_s3_access_key.is_empty()
            || self.media_s3_secret_key.is_empty()
        {
            return Err(ConfigError::Invalid(
                "media S3 bucket and credentials must not be empty".to_owned(),
            ));
        }

        if self.media_max_bytes <= 0
            || self.media_upload_ttl_seconds == 0
            || self.media_download_ttl_seconds == 0
        {
            return Err(ConfigError::Invalid(
                "media limits and TTLs must be positive".to_owned(),
            ));
        }

        for content_type in self.media_allowed_content_types.split(',') {
            if !content_type
                .trim()
                .to_ascii_lowercase()
                .starts_with("image/")
            {
                return Err(ConfigError::Invalid(
                    "MEDIA_ALLOWED_CONTENT_TYPES must contain image types".to_owned(),
                ));
            }
        }

        Ok(())
    }
}
