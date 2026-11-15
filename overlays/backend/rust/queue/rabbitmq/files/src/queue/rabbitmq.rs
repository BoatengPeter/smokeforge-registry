use std::sync::Arc;

use async_trait::async_trait;
use futures_util::StreamExt;
use lapin::{
    BasicProperties, Connection, ConnectionProperties,
    options::{
        BasicAckOptions, BasicConsumeOptions, BasicNackOptions, BasicPublishOptions,
        BasicQosOptions, QueueDeclareOptions,
    },
    types::{AMQPValue, FieldTable},
};

use crate::{
    config::Config,
    error::AppError,
    queue::{
        Deduplicator, Handler, Job, QueueError, Registry,
        codec::{Codec, JsonCodec},
    },
};

enum Processing {
    Complete,
    Requeue,
    Retry(Job),
    Failed(QueueError),
}

const RETRY_DELAYS_MILLIS: [u64; 3] = [1_000, 5_000, 30_000];

pub struct Rabbit {
    channel: lapin::Channel,
    queue: String,
    dlq: String,
    retry_queues: Vec<(String, u64)>,
    max_retries: u32,
    codec: Arc<dyn Codec>,
    deduplicator: Option<Arc<dyn Deduplicator>>,
}

impl Rabbit {
    pub async fn connect(config: &Config) -> Result<Self, AppError> {
        let connection = Connection::connect(&config.rabbitmq_url, ConnectionProperties::default())
            .await
            .map_err(|_| AppError::internal("could not connect to RabbitMQ"))?;
        let channel = connection
            .create_channel()
            .await
            .map_err(|_| AppError::internal("could not open RabbitMQ channel"))?;

        for queue in [&config.rabbitmq_queue, &config.rabbitmq_dlq] {
            channel
                .queue_declare(
                    queue.as_str().into(),
                    QueueDeclareOptions {
                        durable: true,
                        ..QueueDeclareOptions::default()
                    },
                    FieldTable::default(),
                )
                .await
                .map_err(|_| AppError::internal("could not declare RabbitMQ queue"))?;
        }

        let retry_queues = RETRY_DELAYS_MILLIS
            .iter()
            .enumerate()
            .map(|(index, ttl)| {
                (
                    format!("{}.retry.{}", config.rabbitmq_queue, index + 1),
                    *ttl,
                )
            })
            .collect::<Vec<_>>();

        for (queue, ttl) in &retry_queues {
            let mut arguments = FieldTable::default();
            arguments.insert(
                "x-message-ttl".into(),
                AMQPValue::LongUInt(u32::try_from(*ttl).unwrap_or(u32::MAX)),
            );
            arguments.insert(
                "x-dead-letter-exchange".into(),
                AMQPValue::LongString("".into()),
            );
            arguments.insert(
                "x-dead-letter-routing-key".into(),
                AMQPValue::LongString(config.rabbitmq_queue.clone().into()),
            );

            channel
                .queue_declare(
                    queue.as_str().into(),
                    QueueDeclareOptions {
                        durable: true,
                        ..QueueDeclareOptions::default()
                    },
                    arguments,
                )
                .await
                .map_err(|_| AppError::internal("could not declare RabbitMQ retry queue"))?;
        }

        Ok(Self {
            channel,
            queue: config.rabbitmq_queue.clone(),
            dlq: config.rabbitmq_dlq.clone(),
            retry_queues,
            max_retries: config.rabbitmq_max_retries,
            codec: Arc::new(JsonCodec),
            deduplicator: None,
        })
    }

    pub fn with_codec(mut self, codec: Arc<dyn Codec>) -> Self {
        self.codec = codec;
        self
    }

    pub fn with_deduplicator(mut self, deduplicator: Arc<dyn Deduplicator>) -> Self {
        self.deduplicator = Some(deduplicator);
        self
    }

    pub async fn publish(&self, job: &Job) -> Result<(), QueueError> {
        let body = self.codec.encode(job)?;
        self.channel
            .basic_publish(
                "".into(),
                self.queue.as_str().into(),
                BasicPublishOptions::default(),
                &body,
                BasicProperties::default().with_delivery_mode(2),
            )
            .await
            .map_err(|error| QueueError::Message(error.to_string()))?
            .await
            .map_err(|error| QueueError::Message(error.to_string()))?;

        Ok(())
    }

    pub async fn run(&self, registry: Arc<Registry>, prefetch: u16) -> Result<(), AppError> {
        self.channel
            .basic_qos(prefetch, BasicQosOptions::default())
            .await
            .map_err(|_| AppError::internal("could not set RabbitMQ prefetch"))?;

        let mut consumer = self
            .channel
            .basic_consume(
                self.queue.as_str().into(),
                "smokeforge-worker".into(),
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await
            .map_err(|_| AppError::internal("could not consume RabbitMQ queue"))?;

        while let Some(delivery) = consumer.next().await {
            let delivery = delivery.map_err(|_| AppError::internal("RabbitMQ delivery failed"))?;
            let outcome = match self.codec.decode(&delivery.data) {
                Ok(job) => self.process(&registry, job).await,
                Err(error) => Processing::Failed(error),
            };

            match outcome {
                Processing::Complete => {
                    let _ = delivery.ack(BasicAckOptions::default()).await;
                }
                Processing::Requeue => {
                    let _ = delivery
                        .nack(BasicNackOptions {
                            requeue: true,
                            multiple: false,
                        })
                        .await;
                }
                Processing::Retry(job) => match self.publish_retry(&job).await {
                    Ok(()) => {
                        let _ = delivery.ack(BasicAckOptions::default()).await;
                    }
                    Err(error) => {
                        tracing::warn!(error = %error, "job retry publish failed; requeueing");
                        let _ = delivery
                            .nack(BasicNackOptions {
                                requeue: true,
                                multiple: false,
                            })
                            .await;
                    }
                },
                Processing::Failed(error) => {
                    tracing::warn!(error = %error, "job failed; dead-lettering");
                    let _ = delivery
                        .nack(BasicNackOptions {
                            requeue: false,
                            multiple: false,
                        })
                        .await;
                    let _ = self.publish_to_dlq(&delivery.data).await;
                }
            }
        }

        Ok(())
    }

    async fn process(&self, registry: &Registry, job: Job) -> Processing {
        let Some(handler) = registry.lookup(&job.kind) else {
            return Processing::Failed(QueueError::Message("unknown job type".to_owned()));
        };

        if let Some(deduplicator) = &self.deduplicator {
            match deduplicator.seen(&job).await {
                Ok(true) => {
                    tracing::debug!(job_id = %job.id, "duplicate job acknowledged");

                    return Processing::Complete;
                }
                Ok(false) => {}
                Err(error) => {
                    tracing::warn!(
                        job_id = %job.id,
                        error = %error,
                        "dedupe check failed; requeueing"
                    );

                    return Processing::Requeue;
                }
            }
        }

        let marker_job = job.clone();

        match handler.handle(job.clone()).await {
            Ok(()) => {
                if let Some(deduplicator) = &self.deduplicator
                    && let Err(error) = deduplicator.mark(&marker_job).await
                {
                    tracing::warn!(
                        job_id = %marker_job.id,
                        error = %error,
                        "dedupe mark failed"
                    );
                }

                Processing::Complete
            }
            Err(error) if job.attempt < self.max_retries => {
                tracing::warn!(
                    job_id = %job.id,
                    attempt = job.attempt + 1,
                    error = %error,
                    "job failed; scheduling retry"
                );
                Processing::Retry(job)
            }
            Err(error) => Processing::Failed(error),
        }
    }

    async fn publish_retry(&self, job: &Job) -> Result<(), QueueError> {
        let attempt = job.attempt.saturating_add(1);
        let index = usize::try_from(attempt.saturating_sub(1))
            .unwrap_or(usize::MAX)
            .min(self.retry_queues.len().saturating_sub(1));
        let queue = &self.retry_queues[index].0;

        let mut retry = job.clone();
        retry.attempt = attempt;

        let body = self.codec.encode(&retry)?;
        self.channel
            .basic_publish(
                "".into(),
                queue.as_str().into(),
                BasicPublishOptions::default(),
                &body,
                BasicProperties::default().with_delivery_mode(2),
            )
            .await
            .map_err(|error| QueueError::Message(error.to_string()))?
            .await
            .map_err(|error| QueueError::Message(error.to_string()))?;

        Ok(())
    }

    async fn publish_to_dlq(&self, body: &[u8]) -> Result<(), QueueError> {
        self.channel
            .basic_publish(
                "".into(),
                self.dlq.as_str().into(),
                BasicPublishOptions::default(),
                body,
                BasicProperties::default().with_delivery_mode(2),
            )
            .await
            .map_err(|error| QueueError::Message(error.to_string()))?
            .await
            .map_err(|error| QueueError::Message(error.to_string()))?;

        Ok(())
    }
}

pub struct HeartbeatHandler;

#[async_trait]
impl Handler for HeartbeatHandler {
    async fn handle(&self, job: Job) -> Result<(), QueueError> {
        tracing::info!(job_id = %job.id, "queue heartbeat received");

        Ok(())
    }
}
