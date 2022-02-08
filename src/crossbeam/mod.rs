use std::time::{Duration, Instant};

use crossbeam_channel::{SendError, SendTimeoutError, Sender, TrySendError};

pub mod distinct_until_changed;

pub use distinct_until_changed::DistinctUntilChanged;

use crate::PipelineStage;

impl<T> PipelineStage<T, T> for Sender<T> {
    fn apply(&self, element: T) -> Option<T> {
        Some(element)
    }
}

/// The sending side of a channel.
///
/// This might be either a raw [`Sender`] object or a part of a pipeline.
///
/// To learn more, see [`Sender`].
pub trait CrossbeamSender<T>
where
    Self: Sized + PipelineStage<T, T>,
{
    fn try_send(&self, msg: T) -> Result<(), TrySendError<T>>;
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
