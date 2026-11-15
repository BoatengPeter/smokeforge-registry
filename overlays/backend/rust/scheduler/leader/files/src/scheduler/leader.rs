//! Redis-backed scheduler leader election.

use std::{future::Future, pin::Pin, sync::LazyLock, time::Duration};

use redis::{Script, aio::ConnectionManager};
use thiserror::Error;
use tokio::time::{Instant, MissedTickBehavior, interval_at};
use uuid::Uuid;

use crate::{config::Config, queue::QueueError};

static RENEW_SCRIPT: LazyLock<Script> = LazyLock::new(|| {
    Script::new(
        r#"
if redis.call("get", KEYS[1]) == ARGV[1] then
  return redis.call("pexpire", KEYS[1], ARGV[2])
end
return 0
"#,
    )
});

static RELEASE_SCRIPT: LazyLock<Script> = LazyLock::new(|| {
    Script::new(
        r#"
if redis.call("get", KEYS[1]) == ARGV[1] then
  return redis.call("del", KEYS[1])
end
return 0
"#,
    )
});

/// Redis leader-election errors.
#[derive(Debug, Error)]
pub enum LeaderError {
    #[error("redis configuration: {0}")]
    Config(String),
    #[error("redis operation: {0}")]
    Redis(#[from] redis::RedisError),
    #[error("scheduler queue: {0}")]
    Queue(#[from] QueueError),
    #[error("scheduler leader: {0}")]
    Message(String),
}

/// A renewable Redis lease that allows one scheduler replica to lead.
#[derive(Clone)]
pub struct RedisLeader {
    connection: ConnectionManager,
    key: String,
    ttl_seconds: u64,
    renew: Duration,
    retry: Duration,
}

impl RedisLeader {
    pub async fn connect(config: &Config) -> Result<Self, LeaderError> {
        let client = redis::Client::open(config.redis_url.as_str())
            .map_err(|error| LeaderError::Config(error.to_string()))?;
        let connection = ConnectionManager::new(client).await?;

        Ok(Self {
            connection,
            key: leader_key(&config.redis_key_prefix, &config.service_name),
            ttl_seconds: config.scheduler_lease_ttl_seconds,
            renew: Duration::from_secs(config.scheduler_lease_renew_seconds),
            retry: Duration::from_secs(config.scheduler_lease_retry_seconds),
        })
    }

    /// Runs `leader` only while this process holds the lease.
    ///
    /// The function retries acquisition until `shutdown` completes, renews the
    /// lease in the background, and cancels the leader future if the lease is
    /// lost or renewal fails.
    pub async fn run<F, Fut, S>(&self, shutdown: S, mut leader: F) -> Result<(), LeaderError>
    where
        F: FnMut() -> Fut,
        Fut: Future<Output = Result<(), LeaderError>>,
        S: Future<Output = ()>,
    {
        tokio::pin!(shutdown);

        loop {
            let token = Uuid::new_v4().to_string();

            let acquired = tokio::select! {
                () = shutdown.as_mut() => return Ok(()),
                result = self.acquire(&token) => result,
            };

            match acquired {
                Ok(true) => {}
                Ok(false) => {
                    if !self.wait_before_retry(&mut shutdown).await {
                        return Ok(());
                    }

                    continue;
                }
                Err(error) => {
                    tracing::warn!(error = %error, "scheduler lease acquire failed");

                    if !self.wait_before_retry(&mut shutdown).await {
                        return Ok(());
                    }

                    continue;
                }
            }

            tracing::info!("scheduler leadership acquired");

            let outcome = self.run_leader(&token, &mut shutdown, &mut leader).await;

            if let Err(error) = self.release(&token).await {
                tracing::warn!(error = %error, "scheduler lease release failed");
            }

            match outcome {
                Ok(LeaderOutcome::Shutdown) => return Ok(()),
                Ok(LeaderOutcome::Lost) => {
                    tracing::warn!("scheduler leadership lost");

                    if !self.wait_before_retry(&mut shutdown).await {
                        return Ok(());
                    }
                }
                Ok(LeaderOutcome::Completed) => return Ok(()),
                Ok(LeaderOutcome::Failed(error)) => return Err(error),
                Err(error) => return Err(error),
            }
        }
    }

    async fn acquire(&self, token: &str) -> Result<bool, LeaderError> {
        let mut connection = self.connection.clone();
        let result: Option<String> = redis::cmd("SET")
            .arg(self.key.as_str())
            .arg(token)
            .arg("NX")
            .arg("PX")
            .arg(lease_millis(self.ttl_seconds))
            .query_async(&mut connection)
            .await?;

        Ok(result.is_some())
    }

    async fn run_leader<F, Fut, S>(
        &self,
        token: &str,
        shutdown: &mut Pin<&mut S>,
        leader: &mut F,
    ) -> Result<LeaderOutcome, LeaderError>
    where
        F: FnMut() -> Fut,
        Fut: Future<Output = Result<(), LeaderError>>,
        S: Future<Output = ()>,
    {
        let mut renewal = tokio::spawn(self.clone().renew_lease(token.to_owned()));
        let leader_future = leader();
        tokio::pin!(leader_future);

        let (outcome, renewal_finished) = tokio::select! {
            result = &mut leader_future => {
                match result {
                    Ok(()) => (LeaderOutcome::Completed, false),
                    Err(error) => (LeaderOutcome::Failed(error), false),
                }
            }
            () = shutdown.as_mut() => (LeaderOutcome::Shutdown, false),
            result = &mut renewal => {
                match result {
                    Ok(Ok(())) => (LeaderOutcome::Lost, true),
                    Ok(Err(error)) => {
                        tracing::warn!(error = %error, "scheduler lease renewal failed");
                        (LeaderOutcome::Lost, true)
                    }
                    Err(error) => {
                        tracing::warn!(error = %error, "scheduler lease renewal task failed");
                        (LeaderOutcome::Lost, true)
                    }
                }
            }
        };

        if !renewal_finished {
            renewal.abort();
            let _ = renewal.await;
        }

        Ok(outcome)
    }

    async fn renew_lease(self, token: String) -> Result<(), LeaderError> {
        let mut ticker = interval_at(Instant::now() + self.renew, self.renew);
        ticker.set_missed_tick_behavior(MissedTickBehavior::Delay);

        loop {
            ticker.tick().await;

            let mut connection = self.connection.clone();
            let renewed: i32 = RENEW_SCRIPT
                .key(self.key.as_str())
                .arg(token.as_str())
                .arg(lease_millis(self.ttl_seconds))
                .invoke_async(&mut connection)
                .await?;

            if renewed == 0 {
                return Ok(());
            }
        }
    }

    async fn release(&self, token: &str) -> Result<(), LeaderError> {
        let mut connection = self.connection.clone();
        let released: i32 = tokio::time::timeout(
            self.renew,
            RELEASE_SCRIPT
                .key(self.key.as_str())
                .arg(token)
                .invoke_async(&mut connection),
        )
        .await
        .map_err(|_| LeaderError::Message("scheduler lease release timed out".to_owned()))??;

        let _ = released;

        Ok(())
    }

    async fn wait_before_retry<S>(&self, shutdown: &mut Pin<&mut S>) -> bool
    where
        S: Future<Output = ()>,
    {
        tokio::select! {
            () = shutdown.as_mut() => false,
            () = tokio::time::sleep(self.retry) => true,
        }
    }
}

enum LeaderOutcome {
    Shutdown,
    Lost,
    Completed,
    Failed(LeaderError),
}

fn lease_millis(ttl_seconds: u64) -> u64 {
    ttl_seconds.saturating_mul(1000).max(1)
}

fn leader_key(configured: &str, service_name: &str) -> String {
    let configured = configured.trim();
    let service_name = service_name.trim();
    let prefix = if configured.is_empty() {
        service_name
    } else {
        configured
    };

    format!("{}:scheduler:leader", prefix.trim_end_matches(':'))
}

#[cfg(test)]
mod tests {
    use super::{leader_key, lease_millis};

    #[test]
    fn uses_service_name_when_prefix_is_blank() {
        assert_eq!(leader_key("   ", "scheduler"), "scheduler:scheduler:leader");
    }

    #[test]
    fn trims_trailing_colons_from_prefix() {
        assert_eq!(
            leader_key("tenant:", "scheduler"),
            "tenant:scheduler:leader"
        );
    }

    #[test]
    fn converts_ttl_seconds_to_milliseconds() {
        assert_eq!(lease_millis(30), 30_000);
    }

    #[test]
    fn ttl_milliseconds_saturate_instead_of_wrapping() {
        assert_eq!(lease_millis(u64::MAX), u64::MAX);
    }
}
