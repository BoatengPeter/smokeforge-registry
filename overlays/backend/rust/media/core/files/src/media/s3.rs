use hmac::{Hmac, KeyInit, Mac};
use reqwest::{Client, Method, Url};
use sha2::{Digest, Sha256};
use time::{OffsetDateTime, macros::format_description};

use crate::config::Config;
use crate::error::AppError;

const ALGORITHM: &str = "AWS4-HMAC-SHA256";
const UNSIGNED: &str = "UNSIGNED-PAYLOAD";

pub struct S3 {
    internal: Url,
    public: Url,
    bucket: String,
    region: String,
    access: String,
    secret: String,
    client: Client,
}

impl S3 {
    pub fn new(config: &Config) -> Result<Self, AppError> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(config.media_timeout_seconds))
            .build()
            .map_err(|_| AppError::internal("could not create media HTTP client"))?;

        Ok(Self {
            internal: Url::parse(&config.media_s3_endpoint)
                .map_err(|_| AppError::internal("MEDIA_S3_ENDPOINT is invalid"))?,
            public: Url::parse(&config.media_s3_public_endpoint)
                .map_err(|_| AppError::internal("MEDIA_S3_PUBLIC_ENDPOINT is invalid"))?,
            bucket: config.media_s3_bucket.clone(),
            region: config.media_s3_region.clone(),
            access: config.media_s3_access_key.clone(),
            secret: config.media_s3_secret_key.clone(),
            client,
        })
    }

    pub fn presign_put(&self, key: &str, ttl: u64) -> Result<String, AppError> {
        self.presign(Method::PUT, key, ttl, &self.public)
    }

    pub fn presign_get(&self, key: &str, ttl: u64) -> Result<String, AppError> {
        self.presign(Method::GET, key, ttl, &self.public)
    }

    pub async fn head(&self, key: &str) -> Result<(i64, String), AppError> {
        let request = self.signed_request(Method::HEAD, key)?;
        let response = self
            .client
            .execute(request)
            .await
            .map_err(|_| AppError::internal("could not inspect media object"))?;

        if response.status() == reqwest::StatusCode::NOT_FOUND {
            return Err(AppError::new(
                reqwest::StatusCode::UNPROCESSABLE_ENTITY,
                "upload_incomplete",
                "Upload incomplete",
                "The uploaded object does not exist.",
            ));
        }

        if !response.status().is_success() {
            return Err(AppError::internal("could not inspect media object"));
        }

        let size = response.content_length().unwrap_or(0) as i64;
        let content_type = response
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|value| value.to_str().ok())
            .unwrap_or("")
            .to_owned();

        Ok((size, content_type))
    }

    pub async fn delete(&self, key: &str) -> Result<(), AppError> {
        let request = self.signed_request(Method::DELETE, key)?;
        let response = self
            .client
            .execute(request)
            .await
            .map_err(|_| AppError::internal("could not delete media object"))?;

        if !response.status().is_success() && response.status() != reqwest::StatusCode::NOT_FOUND {
            return Err(AppError::internal("could not delete media object"));
        }

        Ok(())
    }

    fn signed_request(&self, method: Method, key: &str) -> Result<reqwest::Request, AppError> {
        let mut url = self.internal.clone();
        url.set_path(&format!("/{}/{}", self.bucket, key));
        let host = url.host_str().unwrap_or_default().to_owned();
        let now = OffsetDateTime::now_utc();
        let amz_date = now
            .format(format_description!(
                "[year][month][day]T[hour][minute][second]Z"
            ))
            .map_err(|_| AppError::internal("could not format signing time"))?;
        let date = now
            .format(format_description!("[year][month][day]"))
            .map_err(|_| AppError::internal("could not format signing date"))?;
        let scope = format!("{date}/{}/s3/aws4_request", self.region);
        let headers =
            format!("host:{host}\nx-amz-content-sha256:{UNSIGNED}\nx-amz-date:{amz_date}\n");
        let signed = "host;x-amz-content-sha256;x-amz-date";
        let canonical = format!(
            "{method}\n{}\n\n{headers}\n{signed}\n{UNSIGNED}",
            url.path()
        );
        let string = format!(
            "AWS4-HMAC-SHA256\n{amz_date}\n{scope}\n{}",
            hex_sha256(&canonical)
        );
        let signature = self.sign(&date, &string)?;
        let authorization = format!(
            "{ALGORITHM} Credential={}/{scope}, SignedHeaders={signed}, Signature={signature}",
            self.access
        );

        self.client
            .request(method, url)
            .header("authorization", authorization)
            .header("x-amz-content-sha256", UNSIGNED)
            .header("x-amz-date", amz_date)
            .build()
            .map_err(|_| AppError::internal("could not build media request"))
    }

    fn presign(&self, method: Method, key: &str, ttl: u64, base: &Url) -> Result<String, AppError> {
        let mut url = base.clone();
        url.set_path(&format!("/{}/{}", self.bucket, key));
        let host = url.host_str().unwrap_or_default().to_owned();
        let now = OffsetDateTime::now_utc();
        let amz_date = now
            .format(format_description!(
                "[year][month][day]T[hour][minute][second]Z"
            ))
            .map_err(|_| AppError::internal("could not format signing time"))?;
        let date = now
            .format(format_description!("[year][month][day]"))
            .map_err(|_| AppError::internal("could not format signing date"))?;
        let scope = format!("{date}/{}/s3/aws4_request", self.region);
        let credential = format!("{}/{scope}", self.access);
        let params = format!(
            "X-Amz-Algorithm={ALGORITHM}&X-Amz-Credential={}&X-Amz-Date={amz_date}&X-Amz-Expires={ttl}&X-Amz-SignedHeaders=host",
            percent(&credential)
        );
        let canonical = format!(
            "{method}\n{}\n{params}\nhost:{host}\n\nhost\n{UNSIGNED}",
            url.path()
        );
        let string = format!(
            "AWS4-HMAC-SHA256\n{amz_date}\n{scope}\n{}",
            hex_sha256(&canonical)
        );
        let signature = self.sign(&date, &string)?;
        url.set_query(Some(&format!("{params}&X-Amz-Signature={signature}")));

        Ok(url.to_string())
    }

    fn sign(&self, date: &str, string: &str) -> Result<String, AppError> {
        let k_date = hmac(format!("AWS4{}", self.secret).as_bytes(), date.as_bytes())?;
        let k_region = hmac(&k_date, self.region.as_bytes())?;
        let k_service = hmac(&k_region, b"s3")?;
        let k_signing = hmac(&k_service, b"aws4_request")?;

        Ok(hex::encode(hmac(&k_signing, string.as_bytes())?))
    }
}

fn hmac(key: &[u8], data: &[u8]) -> Result<Vec<u8>, AppError> {
    let mut mac = Hmac::<Sha256>::new_from_slice(key)
        .map_err(|_| AppError::internal("media signing failed"))?;
    mac.update(data);

    Ok(mac.finalize().into_bytes().to_vec())
}

fn hex_sha256(value: &str) -> String {
    hex::encode(Sha256::digest(value.as_bytes()))
}

fn percent(value: &str) -> String {
    value
        .bytes()
        .map(|byte| match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                (byte as char).to_string()
            }
            _ => format!("%{byte:02X}"),
        })
        .collect()
}
