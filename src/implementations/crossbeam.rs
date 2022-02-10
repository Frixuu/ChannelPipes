//! Implementation of channel_pipes for the `crossbeam_channel` crate.

use crate::{pipeline::IdentityPipelineStage, IntoPiped, PipedSender};
use crossbeam_channel::{SendError, SendTimeoutError, Sender, TrySendError};
use std::time::{Duration, Instant};

impl<T, V> IntoPiped<T, Sender<T>, V> for (Sender<T>, V)
where
    T: Clone + 'static,
{
    fn pipe(self) -> (PipedSender<T, T, Sender<T>>, V) {
        let (s, r) = self;
        (
            PipedSender {
                pipeline: Box::new(IdentityPipelineStage::new()),
                inner_sender: s,
            },
            r,
        )
    }
}

/// The sending side of a channel which is a part of a pipeline.
///
/// To learn more, see [`Sender`] or [`PipedSender`].
pub trait CrossbeamSender<I, T>
where
    Self: Sized,
{
    /// Attempts to send a message into the channel without blocking.
    fn try_send(&self, msg: I) -> Result<(), TrySendError<T>>;
    /// Blocks the current thread until a message is sent or the channel is disconnected.
    fn send(&self, msg: I) -> Result<(), SendError<T>>;
    fn send_timeout(&self, msg: I, timeout: Duration) -> Result<(), SendTimeoutError<T>>;
    fn send_deadline(&self, msg: I, deadline: Instant) -> Result<(), SendTimeoutError<T>>;
    fn is_empty(&self) -> bool;
    fn is_full(&self) -> bool;
    fn len(&self) -> usize;
    fn capacity(&self) -> Option<usize>;
    fn same_channel(&self, other: &Sender<T>) -> bool;
}

impl<I, T> CrossbeamSender<I, T> for PipedSender<I, T, Sender<T>> {
    fn try_send(&self, msg: I) -> Result<(), TrySendError<T>> {
        if let Some(result) = self.pipeline.apply(msg) {
            self.inner_sender.try_send(result)
        } else {
            Ok(())
        }
    }

    fn send(&self, msg: I) -> Result<(), SendError<T>> {
        if let Some(result) = self.pipeline.apply(msg) {
            self.inner_sender.send(result)
        } else {
            Ok(())
        }
    }

    fn send_timeout(&self, msg: I, timeout: Duration) -> Result<(), SendTimeoutError<T>> {
        if let Some(result) = self.pipeline.apply(msg) {
            self.inner_sender.send_timeout(result, timeout)
        } else {
            Ok(())
        }
    }

    fn send_deadline(&self, msg: I, deadline: Instant) -> Result<(), SendTimeoutError<T>> {
        if let Some(result) = self.pipeline.apply(msg) {
            self.inner_sender.send_deadline(result, deadline)
        } else {
            Ok(())
        }
    }

    fn is_empty(&self) -> bool {
        self.inner_sender.is_empty()
    }

    fn is_full(&self) -> bool {
        self.inner_sender.is_full()
    }

    fn len(&self) -> usize {
        self.inner_sender.len()
    }

    fn capacity(&self) -> Option<usize> {
        self.inner_sender.capacity()
    }

    fn same_channel(&self, other: &Sender<T>) -> bool {
        self.inner_sender.same_channel(other)
    }
}

#[cfg(test)]
mod tests {

    use crossbeam_channel::unbounded;

    use crate::{operators::DistinctUntilChanged, CrossbeamSender, IntoPiped};

    #[test]
    fn pipe_is_correctly_created() {
        let (s, r) = unbounded::<i32>().pipe().distinct_until_changed();

        let vec = vec![1, 2, 2, 3, 3, 3, 1];
        for i in vec {
            let _ = s.send(i);
        }

        assert_eq!(Ok(1), r.try_recv());
        assert_eq!(Ok(2), r.try_recv());
        assert_eq!(Ok(3), r.try_recv());
        assert_eq!(Ok(1), r.try_recv());
        assert!(r.try_recv().is_err());
    }
}
