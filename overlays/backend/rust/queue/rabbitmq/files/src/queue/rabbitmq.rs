use std::sync::Arc;

use async_trait::async_trait;
use futures_util::StreamExt;
use lapin::{
    BasicProperties, Connection, ConnectionProperties,
    options::{
        BasicAckOptions, BasicConsumeOptions, BasicNackOptions, BasicPublishOptions,
        BasicQosOptions, QueueDeclareOptions,
    },
    types::FieldTable,
};

use crate::{
    config::Config,
    error::AppError,
    queue::{Handler, Job, QueueError, Registry},
};

pub struct Rabbit {
    channel: lapin::Channel,
    queue: String,
    dlq: String,
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

        Ok(Self {
            channel,
            queue: config.rabbitmq_queue.clone(),
            dlq: config.rabbitmq_dlq.clone(),
        })
    }

    pub async fn publish(&self, job: &Job) -> Result<(), QueueError> {
        let body = job.encode()?;
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
            let job = Job::decode(&delivery.data);

            let result = match job {
                Ok(job) => match registry.lookup(&job.kind) {
                    Some(handler) => handler.handle(job).await,
                    None => Err(QueueError::Message("unknown job type".to_owned())),
                },
                Err(error) => Err(error),
            };

            match result {
                Ok(()) => {
                    let _ = delivery.ack(BasicAckOptions::default()).await;
                }
                Err(error) => {
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
