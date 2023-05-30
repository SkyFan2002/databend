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

use common_vector::index::MetricType;
use common_vector::index::VectorIndex;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CreateVectorIndexPlan {
    pub catalog: String,
    pub database: String,
    pub table: String,
    pub column: String,
    pub vector_index: VectorIndex,
    pub metric_type: MetricType,
}
