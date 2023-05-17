use std::any::Any;
use std::sync::Arc;

use common_exception::Result;
use common_expression::DataBlock;
use common_pipeline_core::processors::port::InputPort;
use common_pipeline_core::processors::port::OutputPort;
use common_pipeline_core::processors::processor::Event;
use common_pipeline_core::processors::Processor;

pub struct TransformIndexKnn {
    input: Arc<InputPort>,
    output: Arc<OutputPort>,
    limit: usize,
    finished: bool,
}

impl TransformIndexKnn {
    pub fn try_create(
        input: Arc<InputPort>,
        output: Arc<OutputPort>,
        limit: usize,
    ) -> Result<Box<dyn Processor>> {
        Ok(Box::new(TransformIndexKnn {
            input,
            output,
            limit,
            finished: false,
        }))
    }
}

#[async_trait::async_trait]
impl Processor for TransformIndexKnn {
    fn name(&self) -> String {
        "TransformIndexKnn".to_string()
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }

    fn event(&mut self) -> Result<Event> {
        if !self.finished {
            return Ok(Event::Sync);
        }
        self.input.finish();
        self.output.finish();
        Ok(Event::Finished)
    }

    fn process(&mut self) -> Result<()> {
        self.finished = true;
        Ok(())
    }
}
