#![allow(
    clippy::wildcard_imports,
    reason = "GPUI Kit exports the framework prelude."
)]

//! Shared UI components and the Smokeforge theme.

pub mod components;
pub mod theme;

pub use components::{EmptyState, PageHeader, SectionCard, StatCard, StatusPage, StatusTone};
