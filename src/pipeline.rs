/// Describes a stage of mapping, filtering etc. of elements passed through a channel.
/// If an element gets promoted to the next stage, returns that element, optionally mapped.
/// If that element does not pass the stage, returns None.
pub trait PipelineStage<T, R> {
    fn apply(&self, element: T) -> Option<R>;
}
