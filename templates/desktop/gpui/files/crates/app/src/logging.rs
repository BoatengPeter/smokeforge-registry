//! Rolling logs and redaction.

use std::io::{self, Write};
use std::path::Path;

use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::EnvFilter;

pub struct LoggingGuard {
    _guard: Option<WorkerGuard>,
}

impl LoggingGuard {
    #[must_use]
    pub const fn disabled() -> Self {
        Self { _guard: None }
    }
}

#[must_use]
pub fn init(log_dir: &Path, level: &str) -> LoggingGuard {
    let _ = std::fs::create_dir_all(log_dir);
    let (writer, guard) =
        tracing_appender::non_blocking(tracing_appender::rolling::daily(log_dir, "app.log"));
    let filter = EnvFilter::try_new(level).unwrap_or_else(|_| EnvFilter::new("info"));
    let file_writer = writer.clone();
    let subscriber = tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_ansi(false)
        .with_writer(move || RedactingWriter(file_writer.clone()))
        .finish();
    let installed = tracing::subscriber::set_global_default(subscriber).is_ok();
    LoggingGuard {
        _guard: installed.then_some(guard),
    }
}

struct RedactingWriter<W: Write>(W);

impl<W: Write> Write for RedactingWriter<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0
            .write_all(redact(&String::from_utf8_lossy(buf)).as_bytes())?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        self.0.flush()
    }
}

#[must_use]
pub fn redact(message: &str) -> String {
    message
        .split_whitespace()
        .map(|part| {
            let lower = part.to_ascii_lowercase();
            if ["token=", "password=", "secret=", "authorization:"]
                .iter()
                .any(|marker| lower.contains(marker))
            {
                "[redacted]"
            } else {
                part
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn redacts_secret_fields() {
        let result = redact("access_token=abc password=def");
        assert!(!result.contains("abc"));
        assert!(!result.contains("def"));
    }
}
