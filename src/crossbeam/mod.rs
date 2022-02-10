//! Implementation of channel_pipes for the `crossbeam_channel` crate.

use crate::PipelineStage;
use crossbeam_channel::{SendError, SendTimeoutError, Sender, TrySendError};
use std::time::{Duration, Instant};

/// The sending side of a channel.
///
/// This might be either a raw [`Sender`] object or a part of a pipeline.
///
/// To learn more, see [`Sender`].
pub trait CrossbeamSender<T>
where
    Self: Sized,
{
    /// Attempts to send a message into the channel without blocking.
    fn try_send(&self, msg: T) -> Result<(), TrySendError<T>>;
    /// Blocks the current thread until a message is sent or the channel is disconnected.
    fn send(&self, msg: T) -> Result<(), SendError<T>>;
    fn send_timeout(&self, msg: T, timeout: Duration) -> Result<(), SendTimeoutError<T>>;
    fn send_deadline(&self, msg: T, deadline: Instant) -> Result<(), SendTimeoutError<T>>;
    fn is_empty(&self) -> bool;
    fn is_full(&self) -> bool;
    fn len(&self) -> usize;
    fn capacity(&self) -> Option<usize>;
    fn same_channel(&self, other: &Sender<T>) -> bool;
}

impl<T> CrossbeamSender<T> for Sender<T> {
    fn try_send(&self, msg: T) -> Result<(), TrySendError<T>> {
        self.try_send(msg)
    }

    fn send(&self, msg: T) -> Result<(), SendError<T>> {
        self.send(msg)
    }

    fn send_timeout(&self, msg: T, timeout: Duration) -> Result<(), SendTimeoutError<T>> {
        self.send_timeout(msg, timeout)
    }

    fn send_deadline(&self, msg: T, deadline: Instant) -> Result<(), SendTimeoutError<T>> {
        self.send_deadline(msg, deadline)
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn is_full(&self) -> bool {
        self.is_full()
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn capacity(&self) -> Option<usize> {
        self.capacity()
    }

    fn same_channel(&self, other: &Sender<T>) -> bool {
        self.same_channel(other)
    }
}

impl<T> PipelineStage<T, T> for Sender<T> {
    fn apply(&self, element: T) -> Option<T> {
        Some(element)
    }
}
