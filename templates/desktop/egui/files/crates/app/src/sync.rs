//! Minimal background push for the durable outbox.

use std::path::PathBuf;

use api_client::{ApiClient, ApiClientError};
use reqwest::Method;
use serde_json::Value;

use {{.crate_name}}_storage::Database;

#[derive(Debug, Default)]
pub struct SyncReport {
    pub pushed: usize,
    pub conflicts: usize,
    pub retried: usize,
    pub error: Option<String>,
}

pub fn run(db_path: PathBuf, base: String, access_token: String) -> SyncReport {
    let mut report = SyncReport::default();
    let Ok(database) = Database::open(&db_path) else {
        report.error = Some("could not open the local database".to_owned());
        return report;
    };
    let Ok(runtime) = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
    else {
        report.error = Some("could not start the sync runtime".to_owned());
        return report;
    };
    let Ok(mut client) = ApiClient::new(&base) else {
        report.error = Some("invalid API URL".to_owned());
        return report;
    };
    client.set_access_token(Some(access_token));

    let rows = database.pending_outbox(50).unwrap_or_default();
    for row in rows {
        let payload: Value = serde_json::from_str(&row.payload).unwrap_or(Value::Null);
        let version = payload.get("version").and_then(Value::as_i64);
        let (method, path) = match row.operation.as_str() {
            "create" => (Method::POST, "/api/v1/library".to_owned()),
            "update" => (Method::PATCH, format!("/api/v1/library/{}", row.item_id)),
            "delete" => (Method::DELETE, format!("/api/v1/library/{}", row.item_id)),
            _ => {
                report.retried += 1;
                continue;
            }
        };
        match runtime.block_on(client.library_mutation(
            method,
            &path,
            (row.operation != "delete").then_some(&payload),
            &row.idempotency_key,
            version,
        )) {
            Ok(_) => {
                let _ = database.acknowledge_outbox(&row.id);
                report.pushed += 1;
            }
            Err(ApiClientError::Api { status, .. }) if status == reqwest::StatusCode::CONFLICT => {
                let _ = database.record_conflict(&row.item_id, &row.payload, "{}");
                let _ = database.acknowledge_outbox(&row.id);
                report.conflicts += 1;
            }
            Err(error) => {
                let retry = chrono::Utc::now() + chrono::Duration::minutes(1);
                let _ = database.retry_outbox(&row.id, &retry.to_rfc3339());
                report.retried += 1;
                report.error = Some(error.to_string());
            }
        }
    }

    let cursor = database.sync_cursor();
    match runtime.block_on(client.library_changes(cursor)) {
        Ok(value) => {
            if let Some(changes) = value.get("changes").and_then(Value::as_array) {
                for change in changes {
                    let _ = database.apply_remote_change(change);
                }
            }
            if let Some(cursor) = value.get("cursor").and_then(Value::as_i64) {
                let _ = database.set_sync_cursor(cursor);
            }
        }
        Err(error) => report.error = Some(error.to_string()),
    }

    report
}
