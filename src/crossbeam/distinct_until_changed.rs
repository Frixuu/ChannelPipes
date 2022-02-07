use std::sync::Arc;

use crossbeam_channel::Sender;
use parking_lot::Mutex;

use crate::PipelineStage;

use super::CrossbeamSender;

pub trait DistinctUntilChangedPipeSenderExt<T, S>
where
    T: PartialEq + Clone,
    S: CrossbeamSender<T>,
{
    fn distinct_until_changed(self) -> DistinctUntilChangedPipeSender<T, S>;
}

pub trait DistinctUntilChanged<T, S, R>
where
    T: PartialEq + Clone,
    S: CrossbeamSender<T>,
{
    fn distinct_until_changed(self) -> (DistinctUntilChangedPipeSender<T, S>, R);
}

impl<T, S> DistinctUntilChangedPipeSenderExt<T, S> for S
where
    T: PartialEq + Clone,
    S: CrossbeamSender<T>,
{
    /// Wraps crossbeam's Sender so that it sends non-repeating elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use channel_pipes::crossbeam::{CrossbeamSender, DistinctUntilChanged};
    /// use crossbeam_channel::unbounded;
    ///
    /// let (s, r) = unbounded::<i32>().distinct_until_changed();
    ///
    /// let vec = vec![1, 2, 2, 3, 3, 3, 1];
    /// for i in vec {
    ///     s.send(i);
    /// }
    ///
    /// assert_eq!(Ok(1), r.try_recv());
    /// assert_eq!(Ok(2), r.try_recv());
    /// assert_eq!(Ok(3), r.try_recv());
    /// assert_eq!(Ok(1), r.try_recv());
    /// assert!(r.try_recv().is_err());
    /// ```
    fn distinct_until_changed(self) -> DistinctUntilChangedPipeSender<T, S> {
        DistinctUntilChangedPipeSender {
            last_message: Arc::new(Mutex::new(None)),
            inner_sender: self,
        }
    }
}

impl<T, S, R> DistinctUntilChanged<T, S, R> for (S, R)
where
    T: PartialEq + Clone,
    S: CrossbeamSender<T>,
{
    fn distinct_until_changed(self) -> (DistinctUntilChangedPipeSender<T, S>, R) {
        (self.0.distinct_until_changed(), self.1)
    }
}

#[derive(Clone)]
pub struct DistinctUntilChangedPipeSender<T, S>
where
    T: PartialEq + Clone,
    S: CrossbeamSender<T> + Sized,
{
    last_message: Arc<Mutex<Option<T>>>,
    inner_sender: S,
}

impl<T, S> PipelineStage<T> for DistinctUntilChangedPipeSender<T, S>
where
    T: PartialEq + Clone,
    S: CrossbeamSender<T> + Sized,
{
    fn select(&self, el: T) -> Option<T> {
        let inner_select = self.inner_sender.select(el);
        if let Some(element) = inner_select {
            let last_message = self.last_message.lock();
            if last_message.as_ref() == Some(&element) {
                None
            } else {
                Some(element)
            }
        } else {
            None
        }
    }
}

impl<T, S> CrossbeamSender<T> for DistinctUntilChangedPipeSender<T, S>
where
    T: PartialEq + Clone,
    S: CrossbeamSender<T> + Sized,
{
    fn try_send(&self, msg: T) -> Result<(), crossbeam_channel::TrySendError<T>> {
        if let Some(msg) = self.select(msg) {
            let mut last_message = self.last_message.lock();
            *last_message = Some(msg.clone());
            self.inner_sender.try_send(msg)
        } else {
            Ok(())
        }
    }

    fn send(&self, msg: T) -> Result<(), crossbeam_channel::SendError<T>> {
        if let Some(msg) = self.select(msg) {
            let mut last_message = self.last_message.lock();
            *last_message = Some(msg.clone());
            self.inner_sender.send(msg)
        } else {
            Ok(())
        }
    }

    fn send_timeout(
        &self,
        msg: T,
        timeout: std::time::Duration,
    ) -> Result<(), crossbeam_channel::SendTimeoutError<T>> {
        if let Some(msg) = self.select(msg) {
            let mut last_message = self.last_message.lock();
            *last_message = Some(msg.clone());
            self.inner_sender.send_timeout(msg, timeout)
        } else {
            Ok(())
        }
    }

    fn send_deadline(
        &self,
        msg: T,
        deadline: std::time::Instant,
    ) -> Result<(), crossbeam_channel::SendTimeoutError<T>> {
        if let Some(msg) = self.select(msg) {
            let mut last_message = self.last_message.lock();
            *last_message = Some(msg.clone());
            self.inner_sender.send_deadline(msg, deadline)
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
    use crate::crossbeam::CrossbeamSender;

    use super::DistinctUntilChanged;

    #[test]
    fn pipe_can_be_cloned_but_shares_state() {
        let (s1, r1) = crossbeam_channel::unbounded::<i32>().distinct_until_changed();
        let (s2, r2) = (s1.clone(), r1.clone());
        let (s3, r3) = (s1.clone(), r1.clone());

        let _ = s1.send(1);
        let _ = s2.send(1);
        let _ = s3.send(1);
        let _ = s2.send(20);
        let _ = s3.send(20);

        assert_eq!(Ok(1), r2.try_recv());
        assert_eq!(Ok(20), r1.try_recv());
        assert!(r3.try_recv().is_err());
    }
}
