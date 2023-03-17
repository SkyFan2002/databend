use common_exception::Result;

use crate::optimizer::rule::Rule;
use crate::optimizer::rule::TransformResult;
use crate::optimizer::RuleID;
use crate::optimizer::SExpr;
use crate::plans::PatternPlan;
use crate::plans::RelOp;

/// Rewrite Pivot with group by and aggregate if
/// Inout:
///     Scan(Pivot)
/// Output:
///     Aggregate
///        \
///         Scan
///
/// for example:
/// Input:
///     SELECT *
///     FROM monthly_sales
///     PIVOT(SUM(amount) FOR MONTH IN ('JAN', 'FEB', 'MAR', 'APR'))
///     ORDER BY EMPID;
/// Output:
///     SELECT EMPID,
///     SUM_IF(AMOUNT,MONTH = 'JAN') AS JAN,
///     SUM_IF(AMOUNT,MONTH = 'FEB') AS FEB
///     FROM monthly_sales
///     GROUP BY EMPID;
pub struct RuleRewritePivot {
    id: RuleID,
    pattern: SExpr,
}

impl RuleRewritePivot {
    pub fn new() -> Self {
        Self {
            id: RuleID::RewritePivot,
            // Scan
            //  \
            //   *
            pattern: SExpr::create_unary(
                PatternPlan {
                    plan_type: RelOp::DummyTableScan,
                }
                .into(),
                SExpr::create_leaf(
                    PatternPlan {
                        plan_type: RelOp::Pattern,
                    }
                    .into(),
                ),
            ),
        }
    }
}

impl Rule for RuleRewritePivot {
    fn id(&self) -> RuleID {
        self.id
    }

    fn apply(&self, s_expr: &SExpr, state: &mut TransformResult) -> Result<()> {
        todo!()
    }

    fn pattern(&self) -> &SExpr {
        &self.pattern
    }
}
