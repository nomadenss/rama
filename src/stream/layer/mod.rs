//! Rama middleware services that operate directly on [`crate::stream::Stream`] types.
//!
//! Examples are services that can operate directly on a `TCP`, `TLS` or `UDP` stream.

mod tracker;
pub use tracker::{BytesRWTrackerHandle, BytesTrackerLayer, BytesTrackerService};
