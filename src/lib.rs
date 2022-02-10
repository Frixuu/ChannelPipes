//! Performs various operations on broadcast queues.
//!
//! This crate is WIP and currently only works with crossbeam.

#[cfg(feature = "crossbeam")]
pub mod crossbeam;

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

mod operators;
#[derive(Clone)]
pub struct PipedSender<I, R, TSender>
where
    TSender: Clone,
{
    pipeline: Box<dyn PipelineStage<I, R>>,
    inner_sender: TSender,
}

impl<I, R, TSender> PipedSender<I, R, TSender>
where
    I: Clone + 'static,
    R: Clone + 'static,
    TSender: Clone,
{
    pub(crate) fn with_pipeline_stage(
        self,
        new_stage: impl PipelineStage<R, R> + 'static,
    ) -> PipedSender<I, R, TSender> {
        let prev_stages = self.pipeline;
        let composite_stage = CompositePipelineStage::of(prev_stages, Box::new(new_stage));
        PipedSender {
            pipeline: Box::new(composite_stage),
            inner_sender: self.inner_sender,
        }
    }
}
