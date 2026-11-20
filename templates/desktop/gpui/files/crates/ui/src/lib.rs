#![allow(
    clippy::wildcard_imports,
    reason = "GPUI Kit exports the framework prelude."
)]

//! Shared UI components and theme.

pub mod components;

pub use components::{EmptyState, PageHeader, SectionCard, StatCard};
