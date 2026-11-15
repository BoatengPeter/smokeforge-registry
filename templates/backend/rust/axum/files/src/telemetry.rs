use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

use crate::config::Config;

/// Initializes structured JSON tracing.
pub fn init(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let filter = EnvFilter::try_new(&config.log_level)?;

    tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer().json())
        .try_init()?;

    Ok(())
}
