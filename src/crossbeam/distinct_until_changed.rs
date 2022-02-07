use std::cell::RefCell;

use crossbeam_channel::Sender;

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
            last_message: Box::new(RefCell::new(None)),
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

pub struct DistinctUntilChangedPipeSender<T, S>
where
    T: PartialEq + Clone,
    S: CrossbeamSender<T> + Sized,
{
    last_message: Box<RefCell<Option<T>>>,
    inner_sender: S,
}

impl<T, S> CrossbeamSender<T> for DistinctUntilChangedPipeSender<T, S>
where
    T: PartialEq + Clone,
    S: CrossbeamSender<T> + Sized,
{
    fn try_send(&self, msg: T) -> Result<(), crossbeam_channel::TrySendError<T>> {
        if (*self.last_message.borrow()).as_ref() != Some(&msg) {
            self.last_message.replace(Some(msg.clone()));
            self.inner_sender.try_send(msg)
        } else {
            Ok(())
        }
    }

    fn send(&self, msg: T) -> Result<(), crossbeam_channel::SendError<T>> {
        if (*self.last_message.borrow()).as_ref() != Some(&msg) {
            self.last_message.replace(Some(msg.clone()));
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
        if (*self.last_message.borrow()).as_ref() != Some(&msg) {
            self.last_message.replace(Some(msg.clone()));
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
        if (*self.last_message.borrow()).as_ref() != Some(&msg) {
            self.last_message.replace(Some(msg.clone()));
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
