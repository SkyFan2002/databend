// Copyright 2023 Datafuse Labs
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

use common_exception::ErrorCode;

use crate::optimizer::rule::Rule;
use crate::optimizer::RuleID;
use crate::optimizer::SExpr;
use crate::plans::IndexKnn;
use crate::plans::PatternPlan;
use crate::plans::RelOp;
use crate::ScalarExpr;

pub struct RuleUseVectorIndex {
    id: RuleID,
    patterns: Vec<SExpr>,
}

impl RuleUseVectorIndex {
    pub fn new() -> Self {
        Self {
            id: RuleID::UseVectorIndex,
            // Limit
            //   \
            //   Sort
            //     \
            //     EvalScalar
            patterns: vec![SExpr::create_unary(
                PatternPlan {
                    plan_type: RelOp::Limit,
                }
                .into(),
                SExpr::create_unary(
                    PatternPlan {
                        plan_type: RelOp::Sort,
                    }
                    .into(),
                    SExpr::create_unary(
                        PatternPlan {
                            plan_type: RelOp::EvalScalar,
                        }
                        .into(),
                        SExpr::create_leaf(
                            PatternPlan {
                                plan_type: RelOp::Pattern,
                            }
                            .into(),
                        ),
                    ),
                ),
            )],
        }
    }
}

impl Rule for RuleUseVectorIndex {
    fn id(&self) -> crate::optimizer::RuleID {
        self.id
    }

    fn apply(
        &self,
        s_expr: &crate::optimizer::SExpr,
        state: &mut crate::optimizer::rule::TransformResult,
    ) -> common_exception::Result<()> {
        let limit = s_expr.plan.as_limit().unwrap();
        let sort = s_expr.walk_down(1).plan.as_sort().unwrap();
        let eval_scalar = s_expr.walk_down(2).plan.as_eval_scalar().unwrap();
        let is_knn = limit.limit.is_some()
            && limit.offset == 0
            && sort.items.len() == 1
            && sort.items[0].asc;
        if !is_knn {
            state.add_result(s_expr.clone());
            return Ok(());
        }
        let sort_by_idx = eval_scalar
            .items
            .iter()
            .position(|item| item.index == sort.items[0].index)
            .unwrap();
        let sort_by = &eval_scalar.items[sort_by_idx];
        match &sort_by.scalar {
            ScalarExpr::FunctionCall(func) if func.func_name == "cosine_distance" => {
                // TODO judge if index exists
                let mut child = s_expr.walk_down(2).clone();
                child
                    .plan
                    .as_eval_scalar_mut()
                    .unwrap()
                    .items
                    .remove(sort_by_idx);
                let result = SExpr::create_unary(
                    IndexKnn {
                        limit: limit.limit.unwrap(),
                    }
                    .into(),
                    child,
                );
                state.add_result(result);
            }
            _ => {
                state.add_result(s_expr.clone());
            }
        }
        Ok(())
    }

    fn patterns(&self) -> &Vec<crate::optimizer::SExpr> {
        &self.patterns
    }
}
