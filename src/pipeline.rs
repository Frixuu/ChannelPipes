use dyn_clonable::*;
use std::marker::PhantomData;

/// Describes a stage of mapping, filtering etc. of elements passed through a channel.
#[clonable]
pub trait PipelineStage<T, R>: Clone {
    /// If an element gets promoted to the next stage,
    /// returns a result of a mapping function on that element.
    /// If that element does not pass the stage, returns None.
    fn apply(&self, element: T) -> Option<R>;
}

/// Folds two different pipeline stages.
#[derive(Clone)]
pub struct CompositePipelineStage<I, B, R> {
    left: Box<dyn PipelineStage<I, B>>,
    right: Box<dyn PipelineStage<B, R>>,
}

impl<I, B, R> CompositePipelineStage<I, B, R> {
    pub fn of(left: Box<dyn PipelineStage<I, B>>, right: Box<dyn PipelineStage<B, R>>) -> Self {
        CompositePipelineStage { left, right }
    }
}

impl<I, B, R> PipelineStage<I, R> for CompositePipelineStage<I, B, R>
where
    I: Clone,
    B: Clone,
    R: Clone,
{
    fn apply(&self, element: I) -> Option<R> {
        self.left
            .apply(element)
            .map(|e| self.right.apply(e))
            .flatten()
    }
}

// A pipeline stage that accepts every element.
#[derive(Copy, Clone)]
pub struct IdentityPipelineStage<T> {
    _marker: PhantomData<T>,
}

impl<T> IdentityPipelineStage<T> {
    pub fn new() -> Self {
        IdentityPipelineStage {
            _marker: PhantomData,
        }
    }
}

impl<T> PipelineStage<T, T> for IdentityPipelineStage<T>
where
    T: Clone,
{
    fn apply(&self, element: T) -> Option<T> {
        Some(element)
    }
}
