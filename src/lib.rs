//! Performs various operations on broadcast queues.
//!
//! This crate is WIP and currently only works with crossbeam.

#[cfg(feature = "crossbeam")]
pub mod crossbeam;

#[cfg(feature = "mpsc")]
compile_error!("channel_pipes doesn't work with std::sync::mpsc yet");

#[cfg(feature = "bus")]
compile_error!("channel_pipes doesn't work with the bus crate yet");

#[cfg(feature = "flume")]
compile_error!("channel_pipes doesn't work with the flume crate yet");

#[cfg(feature = "futures")]
compile_error!("channel_pipes doesn't work with futures-channel yet");

pub trait PipelineStage<T> {
    fn select(&self, el: T) -> Option<T>;
}
