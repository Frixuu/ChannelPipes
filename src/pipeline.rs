use std::marker::PhantomData;

/// Describes a stage of mapping, filtering etc. of elements passed through a channel.
pub trait PipelineStage<T, R> {
    /// If an element gets promoted to the next stage,
    /// returns a result of a mapping function on that element.
    /// If that element does not pass the stage, returns None.
    fn apply(&self, element: T) -> Option<R>;
}

/// Folds two different pipeline stages.
struct CompositePipelineStage<I, B, R> {
    left: Box<dyn PipelineStage<I, B>>,
    right: Box<dyn PipelineStage<B, R>>,
}

impl<I, B, R> PipelineStage<I, R> for CompositePipelineStage<I, B, R> {
    fn apply(&self, element: I) -> Option<R> {
        self.left
            .apply(element)
            .map(|e| self.right.apply(e))
            .flatten()
    }
}

// A pipeline stage that accepts every element.
struct IdentityPipelineStage<T> {
    _marker: PhantomData<T>,
}

impl<T> IdentityPipelineStage<T> {
    fn new() -> Self {
        IdentityPipelineStage {
            _marker: PhantomData,
        }
    }
}

impl<T> PipelineStage<T, T> for IdentityPipelineStage<T> {
    fn apply(&self, element: T) -> Option<T> {
        Some(element)
    }
}
