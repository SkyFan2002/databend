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

use chrono::DateTime;
use chrono::Utc;
use databend_common_exception::ErrorCode;
use databend_common_exception::Result;
use databend_common_meta_app::storage::StorageParams;
use opendal::Operator;

/// Identifies the share whose storage credentials should be resolved.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ShareStorageCredentialRequest {
    pub consumer_tenant: String,
    pub consumer_user: String,
    pub provider_tenant: String,
    pub share_name: String,
    pub query_id: String,
}

/// Temporary storage access material for a shared table.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ShareStorageCredential {
    pub storage_params: StorageParams,
    pub expires_at: Option<DateTime<Utc>>,
}

/// OpenDAL operator built from share credentials.
#[derive(Clone)]
pub struct ShareStorageOperator {
    pub operator: Operator,
    pub expires_at: Option<DateTime<Utc>>,
}

/// Kernel-side boundary for resolving storage access to a provider's share.
#[async_trait::async_trait]
pub trait ShareStorageCredentialProvider: Send + Sync {
    async fn get_storage_credential(
        &self,
        request: ShareStorageCredentialRequest,
    ) -> Result<ShareStorageCredential>;

    async fn get_operator(
        &self,
        request: ShareStorageCredentialRequest,
    ) -> Result<ShareStorageOperator> {
        let credential = self.get_storage_credential(request).await?;
        let operator = crate::init_operator(&credential.storage_params)?;
        Ok(ShareStorageOperator {
            operator,
            expires_at: credential.expires_at,
        })
    }
}

/// TODO: replace this with the cloud-control backed provider.
#[derive(Clone, Debug, Default)]
pub struct TodoShareStorageCredentialProvider;

#[async_trait::async_trait]
impl ShareStorageCredentialProvider for TodoShareStorageCredentialProvider {
    async fn get_storage_credential(
        &self,
        _request: ShareStorageCredentialRequest,
    ) -> Result<ShareStorageCredential> {
        Err(ErrorCode::Unimplemented(
            "TODO: resolve data share storage credentials from the cloud sharing service",
        ))
    }
}
