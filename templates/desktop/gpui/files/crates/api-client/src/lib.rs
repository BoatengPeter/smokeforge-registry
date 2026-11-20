//! Typed API client for authentication and the Library contract.

use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum ApiClientError {
    #[error("API mode is not configured")]
    NotConfigured,
    #[error("request failed: {0}")]
    Transport(#[from] reqwest::Error),
    #[error("invalid API URL: {0}")]
    Url(#[from] url::ParseError),
    #[error("API rejected the request ({status}): {message}")]
    Api { status: StatusCode, message: String },
    #[error("not authenticated")]
    Unauthorized,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub role: String,
    pub email_verified: bool,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemDetails {
    #[serde(alias = "detail")]
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct ApiSession {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
}

#[derive(Debug, Clone)]
pub struct ApiClient {
    base: Url,
    http: Client,
    access_token: Option<String>,
    refresh_token: Option<String>,
}

impl ApiClient {
    /// Creates a client rooted at the API origin.
    ///
    /// # Errors
    ///
    /// Returns an error when the URL is invalid or the HTTP client cannot be built.
    pub fn new(base: &str) -> Result<Self, ApiClientError> {
        Ok(Self {
            base: Url::parse(base)?,
            http: Client::builder().build()?,
            access_token: None,
            refresh_token: None,
        })
    }

    pub fn set_access_token(&mut self, token: Option<String>) {
        self.access_token = token;
    }

    pub fn set_session(&mut self, access_token: String, refresh_token: String) {
        self.access_token = Some(access_token);
        self.refresh_token = Some(refresh_token);
    }

    /// Authenticates with email and password.
    ///
    /// # Errors
    ///
    /// Returns an error for transport or API failures.
    pub async fn login(
        &mut self,
        email: &str,
        password: &str,
    ) -> Result<ApiSession, ApiClientError> {
        let response = self
            .http
            .post(self.endpoint("/api/v1/auth/login")?)
            .json(&serde_json::json!({"email": email, "password": password}))
            .send()
            .await?;
        let tokens: TokenResponse = self.decode(response).await?;
        self.access_token = Some(tokens.access_token.clone());
        self.refresh_token = Some(tokens.refresh_token.clone());
        Ok(ApiSession {
            access_token: tokens.access_token,
            refresh_token: tokens.refresh_token,
            expires_in: tokens.expires_in,
        })
    }

    /// Refreshes an access token.
    ///
    /// # Errors
    ///
    /// Returns an error for transport or API failures.
    pub async fn refresh(&mut self, refresh_token: &str) -> Result<ApiSession, ApiClientError> {
        let response = self
            .http
            .post(self.endpoint("/api/v1/auth/refresh")?)
            .json(&serde_json::json!({"refresh_token": refresh_token}))
            .send()
            .await?;
        let tokens: TokenResponse = self.decode(response).await?;
        self.access_token = Some(tokens.access_token.clone());
        self.refresh_token = Some(tokens.refresh_token.clone());
        Ok(ApiSession {
            access_token: tokens.access_token,
            refresh_token: tokens.refresh_token,
            expires_in: tokens.expires_in,
        })
    }

    /// Revokes the refresh token.
    ///
    /// # Errors
    ///
    /// Returns an error for transport or API failures.
    pub async fn logout(&mut self, refresh_token: &str) -> Result<(), ApiClientError> {
        let response = self
            .http
            .post(self.endpoint("/api/v1/auth/logout")?)
            .json(&serde_json::json!({"refresh_token": refresh_token}))
            .send()
            .await?;
        self.access_token = None;
        self.refresh_token = None;
        self.expect_empty(response).await
    }

    /// Loads the current account.
    ///
    /// # Errors
    ///
    /// Returns `Unauthorized` when no access token is available, or an API error.
    pub async fn me(&self) -> Result<User, ApiClientError> {
        let token = self
            .access_token
            .as_deref()
            .ok_or(ApiClientError::Unauthorized)?;
        let response = self
            .http
            .get(self.endpoint("/api/v1/auth/me")?)
            .bearer_auth(token)
            .send()
            .await?;
        self.decode(response).await
    }

    /// Pulls the full Library snapshot.
    ///
    /// # Errors
    ///
    /// Returns an error when unauthenticated or the API rejects the request.
    pub async fn library_snapshot(&mut self) -> Result<serde_json::Value, ApiClientError> {
        self.authorized_json(
            reqwest::Method::GET,
            "/api/v1/library/snapshot",
            None,
            None,
            None,
        )
        .await
    }

    /// Pulls Library changes after a cursor.
    ///
    /// # Errors
    ///
    /// Returns an error when unauthenticated or the API rejects the request.
    pub async fn library_changes(
        &mut self,
        cursor: i64,
    ) -> Result<serde_json::Value, ApiClientError> {
        let path = format!("/api/v1/library/changes?cursor={cursor}");
        self.authorized_json(reqwest::Method::GET, &path, None, None, None)
            .await
    }

    /// Pushes one Library mutation.
    ///
    /// # Errors
    ///
    /// Returns an error when unauthenticated, stale, or rejected by the API.
    pub async fn library_mutation(
        &mut self,
        method: reqwest::Method,
        path: &str,
        body: Option<&serde_json::Value>,
        idempotency_key: &str,
        version: Option<i64>,
    ) -> Result<serde_json::Value, ApiClientError> {
        self.authorized_json(method, path, body, Some(idempotency_key), version)
            .await
    }

    async fn authorized_json(
        &mut self,
        method: reqwest::Method,
        path: &str,
        body: Option<&serde_json::Value>,
        idempotency_key: Option<&str>,
        version: Option<i64>,
    ) -> Result<serde_json::Value, ApiClientError> {
        let mut response = self
            .send_authorized(method.clone(), path, body, idempotency_key, version)
            .await?;
        if response.status() == StatusCode::UNAUTHORIZED {
            let refresh = self
                .refresh_token
                .clone()
                .ok_or(ApiClientError::Unauthorized)?;
            self.refresh(&refresh).await?;
            response = self
                .send_authorized(method, path, body, idempotency_key, version)
                .await?;
        }
        self.decode(response).await
    }

    async fn send_authorized(
        &self,
        method: reqwest::Method,
        path: &str,
        body: Option<&serde_json::Value>,
        idempotency_key: Option<&str>,
        version: Option<i64>,
    ) -> Result<reqwest::Response, ApiClientError> {
        let token = self
            .access_token
            .as_deref()
            .ok_or(ApiClientError::Unauthorized)?;
        let mut request = self
            .http
            .request(method, self.endpoint(path)?)
            .bearer_auth(token);
        if let Some(body) = body {
            request = request.json(body);
        }
        if let Some(key) = idempotency_key {
            request = request.header("Idempotency-Key", key);
        }
        if let Some(version) = version {
            request = request.header(reqwest::header::IF_MATCH, version.to_string());
        }
        Ok(request.send().await?)
    }

    fn endpoint(&self, path: &str) -> Result<Url, ApiClientError> {
        Ok(self.base.join(path)?)
    }

    async fn decode<T: for<'de> Deserialize<'de>>(
        &self,
        response: reqwest::Response,
    ) -> Result<T, ApiClientError> {
        if response.status() == StatusCode::UNAUTHORIZED {
            return Err(ApiClientError::Unauthorized);
        }
        if !response.status().is_success() {
            let status = response.status();
            let message = response
                .json::<ProblemDetails>()
                .await
                .map_or_else(|_| status.to_string(), |problem| problem.message);
            return Err(ApiClientError::Api { status, message });
        }
        Ok(response.json().await?)
    }

    async fn expect_empty(&self, response: reqwest::Response) -> Result<(), ApiClientError> {
        if response.status().is_success() {
            return Ok(());
        }
        let status = response.status();
        let message = response
            .json::<ProblemDetails>()
            .await
            .map_or_else(|_| status.to_string(), |problem| problem.message);
        Err(ApiClientError::Api { status, message })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn client_rejects_invalid_urls() {
        assert!(ApiClient::new("not a url").is_err());
    }
}
