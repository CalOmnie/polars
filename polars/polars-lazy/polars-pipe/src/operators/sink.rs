use std::any::Any;

use super::*;

#[derive(Debug)]
pub enum SinkResult {
    Finished,
    CanHaveMoreInput,
}

pub enum FinalizedSink {
    Finished(DataFrame),
    Operator(Box<dyn Operator>),
}

pub trait Sink: Send + Sync {
    fn sink(&mut self, context: &PExecutionContext, chunk: DataChunk) -> PolarsResult<SinkResult>;

    fn combine(&mut self, other: Box<dyn Sink>);

    fn split(&self, thread_no: usize) -> Box<dyn Sink>;

    fn finalize(&mut self) -> PolarsResult<FinalizedSink>;

    fn as_any(&mut self) -> &mut dyn Any;
}
