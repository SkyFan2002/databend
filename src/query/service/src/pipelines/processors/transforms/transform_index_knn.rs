use std::any::Any;
use std::sync::Arc;

use common_exception::Result;
use common_pipeline_core::processors::port::InputPort;
use common_pipeline_core::processors::port::OutputPort;
use common_pipeline_core::processors::processor::Event;
use common_pipeline_core::processors::Processor;

pub struct TransformIndexKnn {
    input: Arc<InputPort>,
    output: Arc<OutputPort>,
    limit: usize,
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
        }))
    }
}

#[async_trait::async_trait]
impl Processor for TransformIndexKnn {
    fn name(&self) -> String {
        "HashJoinProbe".to_string()
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }

    fn event(&mut self) -> Result<Event> {
        todo!()
    }

    fn interrupt(&self) {
        todo!()
    }

    fn process(&mut self) -> Result<()> {
        todo!()
    }

    #[async_backtrace::framed]
    async fn async_process(&mut self) -> Result<()> {
        todo!()
    }
}
