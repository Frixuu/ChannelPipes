//! Performs various operations on broadcast queues.
//!
//! This crate is WIP and currently only works with crossbeam.

pub(crate) mod implementations;

#[cfg(feature = "crossbeam")]
pub use implementations::crossbeam::CrossbeamSender;

#[cfg(feature = "mpsc")]
compile_error!("channel_pipes doesn't work with std::sync::mpsc yet");

#[cfg(feature = "bus")]
compile_error!("channel_pipes doesn't work with the bus crate yet");

#[cfg(feature = "flume")]
compile_error!("channel_pipes doesn't work with the flume crate yet");

#[cfg(feature = "futures")]
compile_error!("channel_pipes doesn't work with futures-channel yet");

mod pipeline;
use pipeline::CompositePipelineStage;
pub use pipeline::PipelineStage;

pub mod operators;

#[derive(Clone)]
pub struct PipedSender<I, R, S>
where
    S: Clone,
{
    pipeline: Box<dyn PipelineStage<I, R>>,
    inner_sender: S,
}

impl<I, R, S> PipedSender<I, R, S>
where
    I: Clone + 'static,
    R: Clone + 'static,
    S: Clone,
{
    pub(crate) fn with_pipeline_stage(
        self,
        new_stage: impl PipelineStage<R, R> + 'static,
    ) -> PipedSender<I, R, S> {
        let prev_stages = self.pipeline;
        let composite_stage = CompositePipelineStage::of(prev_stages, Box::new(new_stage));
        PipedSender {
            pipeline: Box::new(composite_stage),
            inner_sender: self.inner_sender,
        }
    }
}

trait IntoPiped<T, S, V>
where
    S: Clone,
{
    fn pipe(self) -> (PipedSender<T, T, S>, V);
}
