use std::sync::Arc;

use parking_lot::Mutex;

use crate::{PipedSender, PipelineStage};

#[derive(Clone)]
struct DistinctUntilChangedStage<T>
where
    T: PartialEq + Clone,
{
    previous: Arc<Mutex<Option<T>>>,
}

impl<T> DistinctUntilChangedStage<T>
where
    T: PartialEq + Clone,
{
    fn new() -> Self {
        Self {
            previous: Arc::new(Mutex::new(None)),
        }
    }
}

impl<T> PipelineStage<T, T> for DistinctUntilChangedStage<T>
where
    T: PartialEq + Clone,
{
    fn apply(&self, element: T) -> Option<T> {
        let mut last_element = self.previous.lock();
        if last_element.as_ref() == Some(&element) {
            None
        } else {
            *last_element = Some(element.clone());
            Some(element)
        }
    }
}

pub trait DistinctUntilChanged<I, R, S, V>
where
    S: Clone,
{
    fn distinct_until_changed(self) -> (PipedSender<I, R, S>, V);
}

impl<I, R, S, V> DistinctUntilChanged<I, R, S, V> for (PipedSender<I, R, S>, V)
where
    I: Clone + 'static,
    R: PartialEq + Clone + 'static,
    S: Clone,
{
    fn distinct_until_changed(self) -> (PipedSender<I, R, S>, V) {
        let (s, r) = self;
        (s.with_pipeline_stage(DistinctUntilChangedStage::new()), r)
    }
}
