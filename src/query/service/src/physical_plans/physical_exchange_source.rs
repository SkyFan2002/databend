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

use std::any::Any;

use databend_common_catalog::table_context::TableContext;
use databend_common_exception::Result;
use databend_common_expression::DataSchemaRef;
use databend_common_pipeline::core::PlanScope;

use crate::physical_plans::format::ExchangeSourceFormatter;
use crate::physical_plans::format::PhysicalFormat;
use crate::physical_plans::physical_plan::IPhysicalPlan;
use crate::physical_plans::physical_plan::PhysicalPlan;
use crate::physical_plans::physical_plan::PhysicalPlanMeta;
use crate::pipelines::PipelineBuilder;
use crate::servers::flight::v1::exchange::BroadcastExchangeParams;
use crate::servers::flight::v1::exchange::DataExchange;
use crate::servers::flight::v1::exchange::GlobalExchangeParams;
use crate::servers::flight::v1::exchange::via_broadcast_exchange_source;
use crate::servers::flight::v1::exchange::via_hash_exchange_source;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ExchangeSource {
    pub meta: PhysicalPlanMeta,

    // Output schema of exchanged data
    pub schema: DataSchemaRef,

    // Fragment ID of source fragment
    pub source_fragment_id: usize,
    pub query_id: String,
    pub source_exchange: Option<DataExchange>,
}

#[typetag::serde]
impl IPhysicalPlan for ExchangeSource {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn get_meta(&self) -> &PhysicalPlanMeta {
        &self.meta
    }

    fn get_meta_mut(&mut self) -> &mut PhysicalPlanMeta {
        &mut self.meta
    }

    #[recursive::recursive]
    fn output_schema(&self) -> Result<DataSchemaRef> {
        Ok(self.schema.clone())
    }

    fn formatter(&self) -> Result<Box<dyn PhysicalFormat + '_>> {
        Ok(ExchangeSourceFormatter::create(self))
    }

    fn is_distributed_plan(&self) -> bool {
        true
    }

    fn display_in_profile(&self) -> bool {
        false
    }

    fn derive(&self, children: Vec<PhysicalPlan>) -> PhysicalPlan {
        assert!(children.is_empty());
        PhysicalPlan::new(ExchangeSource {
            meta: self.meta.clone(),
            schema: self.schema.clone(),
            source_fragment_id: self.source_fragment_id,
            query_id: self.query_id.clone(),
            source_exchange: self.source_exchange.clone(),
        })
    }

    fn build_pipeline2(&self, builder: &mut PipelineBuilder) -> Result<()> {
        let exchange_manager = builder.ctx.get_exchange_manager();
        if !exchange_manager.has_local_fragment(&self.query_id, self.source_fragment_id)? {
            return match &self.source_exchange {
                Some(DataExchange::Broadcast(exchange)) => via_broadcast_exchange_source(
                    builder.ctx.clone(),
                    &BroadcastExchangeParams {
                        query_id: self.query_id.clone(),
                        executor_id: builder.ctx.get_cluster().local_id.clone(),
                        schema: self.schema.clone(),
                        exchange_id: exchange.id.clone(),
                        destination_channels: exchange.destination_channels.clone(),
                    },
                    &mut builder.main_pipeline,
                ),
                Some(DataExchange::GlobalShuffleExchange(exchange)) => via_hash_exchange_source(
                    &builder.ctx,
                    &GlobalExchangeParams {
                        query_id: self.query_id.clone(),
                        executor_id: builder.ctx.get_cluster().local_id.clone(),
                        schema: self.schema.clone(),
                        exchange_id: exchange.id.clone(),
                        shuffle_keys: exchange.shuffle_keys.clone(),
                        destination_channels: exchange.destination_channels.clone(),
                    },
                    &mut builder.main_pipeline,
                ),
                Some(source_exchange) => Err(databend_common_exception::ErrorCode::Unimplemented(
                    format!(
                        "ExchangeSource cannot subscribe remote {:?} fragment {}",
                        source_exchange, self.source_fragment_id
                    ),
                )),
                None => Err(databend_common_exception::ErrorCode::Unimplemented(
                    format!(
                        "ExchangeSource {} is missing source exchange metadata",
                        self.source_fragment_id
                    ),
                )),
            };
        }

        let build_res = exchange_manager.get_fragment_source(
            &self.query_id,
            self.source_fragment_id,
            builder.exchange_injector.clone(),
        )?;

        let plan_scope = PlanScope::get_plan_scope();
        let build_pipeline = build_res.main_pipeline.finalize(plan_scope);

        // add sharing data
        builder.join_state = build_res.builder_data.input_join_state;
        builder.merge_into_probe_data_fields = build_res.builder_data.input_probe_schema;

        // Merge pipeline
        assert_eq!(builder.main_pipeline.output_len(), 0);
        let sinks = builder.main_pipeline.merge(build_pipeline)?;
        builder.main_pipeline.extend_sinks(sinks);
        builder.pipelines.extend(build_res.sources_pipelines);
        Ok(())
    }
}
