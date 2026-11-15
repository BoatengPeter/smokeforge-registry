//! Signed imgproxy transform URL builder.

use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use hmac::{Hmac, KeyInit, Mac};
use sha2::Sha256;

use super::Transform;
use crate::{config::Config, error::AppError};

/// Signs imgproxy URLs for a private S3 bucket.
pub struct Imgproxy {
    base_url: String,
    bucket: String,
    key: Vec<u8>,
    salt: Vec<u8>,
    ttl_seconds: u64,
}

impl Imgproxy {
    /// Builds an imgproxy client when a base URL is configured.
    pub fn new(config: &Config) -> Result<Option<Self>, AppError> {
        if config.media_imgproxy_base_url.trim().is_empty() {
            return Ok(None);
        }

        let key = hex::decode(&config.media_imgproxy_key)
            .map_err(|_| AppError::internal("MEDIA_IMGPROXY_KEY must be hex"))?;
        let salt = hex::decode(&config.media_imgproxy_salt)
            .map_err(|_| AppError::internal("MEDIA_IMGPROXY_SALT must be hex"))?;

        if key.is_empty() || salt.is_empty() {
            return Err(AppError::internal(
                "imgproxy key and salt must not be empty",
            ));
        }

        Ok(Some(Self {
            base_url: config
                .media_imgproxy_base_url
                .trim_end_matches('/')
                .to_owned(),
            bucket: config.media_s3_bucket.clone(),
            key,
            salt,
            ttl_seconds: config.media_download_ttl_seconds,
        }))
    }

    /// Builds a signed processing URL for an object key.
    pub fn build_url(&self, object_key: &str, transform: &Transform) -> Result<String, AppError> {
        let options = transform.options()?;
        let source = URL_SAFE_NO_PAD.encode(format!("s3://{}/{object_key}", self.bucket));
        let expires = unix_time().saturating_add(self.ttl_seconds);
        let path = format!("/{}/{}?expires={expires}", options.join("/"), source);
        let signature = self.signature(&path)?;

        Ok(format!("{}/{}{}", self.base_url, signature, path))
    }

    fn signature(&self, path: &str) -> Result<String, AppError> {
        let mut mac = Hmac::<Sha256>::new_from_slice(&self.key)
            .map_err(|_| AppError::internal("imgproxy signing key is invalid"))?;
        mac.update(&self.salt);
        mac.update(path.as_bytes());

        Ok(URL_SAFE_NO_PAD.encode(mac.finalize().into_bytes()))
    }
}

fn unix_time() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;

    fn test_config() -> Config {
        Config {
            media_imgproxy_base_url: "http://localhost:8081".to_owned(),
            media_imgproxy_key: "0123456789abcdef0123456789abcdef".to_owned(),
            media_imgproxy_salt: "fedcba9876543210fedcba9876543210".to_owned(),
            ..Config::default()
        }
    }

    #[test]
    fn builds_signed_transform_url() {
        let config = test_config();
        let builder = Imgproxy::new(&config).unwrap().unwrap();
        let transform = Transform {
            width: 320,
            format: "webp".to_owned(),
            ..Transform::default()
        };

        let url = builder
            .build_url("users/u/media/i.jpg", &transform)
            .unwrap();

        assert!(url.starts_with("http://localhost:8081/"));
        assert!(url.contains("rs:fill:320:0:0"));
        assert!(url.contains("format:webp"));
        assert!(url.contains("expires="));
    }

    #[test]
    fn disabled_without_base_url() {
        let config = Config::default();
        assert!(Imgproxy::new(&config).unwrap().is_none());
    }
}
