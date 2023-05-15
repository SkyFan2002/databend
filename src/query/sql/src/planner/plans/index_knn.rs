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

use crate::optimizer::Distribution;
use crate::optimizer::StatInfo;
use crate::optimizer::Statistics;
use crate::plans::Operator;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IndexKnn {
    pub limit: usize,
}

impl Operator for IndexKnn {
    fn rel_op(&self) -> super::RelOp {
        super::RelOp::IndexKnn
    }

    fn derive_relational_prop(
        &self,
        rel_expr: &crate::optimizer::RelExpr,
    ) -> common_exception::Result<crate::optimizer::RelationalProperty> {
        rel_expr.derive_relational_prop_child(0)
    }

    fn derive_physical_prop(
        &self,
        rel_expr: &crate::optimizer::RelExpr,
    ) -> common_exception::Result<crate::optimizer::PhysicalProperty> {
        rel_expr.derive_physical_prop_child(0)
    }

    fn derive_cardinality(
        &self,
        rel_expr: &crate::optimizer::RelExpr,
    ) -> common_exception::Result<crate::optimizer::StatInfo> {
        let stat_info = rel_expr.derive_cardinality_child(0)?;
        let cardinality = (self.limit as f64).min(stat_info.cardinality);
        Ok(StatInfo {
            cardinality,
            statistics: Statistics {
                precise_cardinality: None,
                column_stats: Default::default(),
            },
        })
    }

    fn compute_required_prop_child(
        &self,
        _ctx: std::sync::Arc<dyn common_catalog::table_context::TableContext>,
        _rel_expr: &crate::optimizer::RelExpr,
        _child_index: usize,
        required: &crate::optimizer::RequiredProperty,
    ) -> common_exception::Result<crate::optimizer::RequiredProperty> {
        let mut required = required.clone();
        required.distribution = Distribution::Serial;
        Ok(required)
    }
}
