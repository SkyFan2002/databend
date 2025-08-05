// Copyright 2021 Datafuse Labs
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::sync::Arc;

use async_channel::Receiver;
use async_channel::Sender;
use databend_common_exception::ErrorCode;
use databend_common_exception::Result;
use databend_common_expression::DataBlock;
use databend_common_pipeline_core::processors::InputPort;
use databend_common_pipeline_core::processors::OutputPort;
use databend_common_pipeline_core::processors::ProcessorPtr;
use databend_common_pipeline_sinks::AsyncSink;
use databend_common_pipeline_sinks::AsyncSinker;
use databend_common_pipeline_sources::AsyncSource;
use databend_common_pipeline_sources::AsyncSourcer;
use databend_common_storages_fuse::TableContext;

pub struct MaterializedCteSink {
    state: Arc<MaterializedCTEState>,
}

impl MaterializedCteSink {
    pub fn create(input: Arc<InputPort>, state: Arc<MaterializedCTEState>) -> Result<ProcessorPtr> {
        Ok(ProcessorPtr::create(AsyncSinker::create(input, Self {
            state,
        })))
    }
}

#[async_trait::async_trait]
impl AsyncSink for MaterializedCteSink {
    const NAME: &'static str = "MaterializedCteSink";

    async fn consume(&mut self, data_block: DataBlock) -> Result<bool> {
        for sender in self.state.senders.iter() {
            sender
                .send(DataBlockWithId {
                    id: self.state.next_block_id.fetch_add(1, Ordering::Relaxed),
                    block: data_block.clone(),
                })
                .await
                .map_err(|_| {
                    ErrorCode::Internal("Failed to send blocks to materialized cte consumer")
                })?;
        }
        Ok(false)
    }

    async fn on_finish(&mut self) -> Result<()> {
        for sender in self.state.senders.iter() {
            sender.close();
        }
        Ok(())
    }
}

pub struct MaterializedCTESource {
    state: Arc<MaterializedCTEState>,
    cte_ref_id: usize,
}

impl MaterializedCTESource {
    pub fn create(
        ctx: Arc<dyn TableContext>,
        output_port: Arc<OutputPort>,
        state: Arc<MaterializedCTEState>,
        cte_ref_id: usize,
    ) -> Result<ProcessorPtr> {
        AsyncSourcer::create(ctx, output_port, Self { state, cte_ref_id })
    }
}

#[async_trait::async_trait]
impl AsyncSource for MaterializedCTESource {
    const NAME: &'static str = "MaterializeCTESource";

    #[async_backtrace::framed]
    async fn generate(&mut self) -> Result<Option<DataBlock>> {
        if let Ok(data) = self.state.receivers[self.cte_ref_id].recv().await {
            return Ok(Some(data.block));
        }
        Ok(None)
    }
}

pub struct MaterializedCTEState {
    senders: Vec<Sender<DataBlockWithId>>,
    receivers: Vec<Receiver<DataBlockWithId>>,
    next_cte_ref_id: AtomicUsize,
    next_block_id: AtomicUsize,
}

impl MaterializedCTEState {
    pub fn new(
        senders: Vec<Sender<DataBlockWithId>>,
        receivers: Vec<Receiver<DataBlockWithId>>,
    ) -> Self {
        Self {
            senders,
            receivers,
            next_cte_ref_id: AtomicUsize::new(0),
            next_block_id: AtomicUsize::new(0),
        }
    }

    pub fn next_cte_ref_id(&self) -> usize {
        self.next_cte_ref_id.fetch_add(1, Ordering::Relaxed)
    }
}

pub struct DataBlockWithId {
    id: usize,
    block: DataBlock,
}
