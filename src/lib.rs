//! Performs various operations on broadcast queues.
//!
//! This crate is WIP and currently only works with crossbeam.

#[cfg(feature = "crossbeam")]
pub mod crossbeam;
