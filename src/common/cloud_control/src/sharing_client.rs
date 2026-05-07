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

use std::sync::Arc;

use tonic::Request;
use tonic::transport::Channel;

use crate::pb::CreateShareRequest;
use crate::pb::CreateShareResponse;
use crate::pb::DropShareRequest;
use crate::pb::DropShareResponse;
use crate::pb::GetShareCredentialRequest;
use crate::pb::GetShareCredentialResponse;
use crate::pb::GrantShareRequest;
use crate::pb::GrantShareResponse;
use crate::pb::RevokeShareRequest;
use crate::pb::RevokeShareResponse;
use crate::pb::sharing_service_client::SharingServiceClient;

pub(crate) const SHARING_CLIENT_VERSION: &str = "v1";
pub(crate) const SHARING_CLIENT_VERSION_NAME: &str = "SHARING_CLIENT_VERSION";

pub struct SharingClient {
    pub client: SharingServiceClient<Channel>,
}

impl SharingClient {
    pub async fn new(channel: Channel) -> databend_common_exception::Result<Arc<SharingClient>> {
        let client = SharingServiceClient::new(channel);
        Ok(Arc::new(SharingClient { client }))
    }

    pub async fn create_share(
        &self,
        req: Request<CreateShareRequest>,
    ) -> databend_common_exception::Result<CreateShareResponse> {
        let mut client = self.client.clone();
        let resp = client.create_share(req).await?;
        Ok(resp.into_inner())
    }

    pub async fn drop_share(
        &self,
        req: Request<DropShareRequest>,
    ) -> databend_common_exception::Result<DropShareResponse> {
        let mut client = self.client.clone();
        let resp = client.drop_share(req).await?;
        Ok(resp.into_inner())
    }

    pub async fn grant_share(
        &self,
        req: Request<GrantShareRequest>,
    ) -> databend_common_exception::Result<GrantShareResponse> {
        let mut client = self.client.clone();
        let resp = client.grant_share(req).await?;
        Ok(resp.into_inner())
    }

    pub async fn revoke_share(
        &self,
        req: Request<RevokeShareRequest>,
    ) -> databend_common_exception::Result<RevokeShareResponse> {
        let mut client = self.client.clone();
        let resp = client.revoke_share(req).await?;
        Ok(resp.into_inner())
    }

    pub async fn get_share_credential(
        &self,
        req: Request<GetShareCredentialRequest>,
    ) -> databend_common_exception::Result<GetShareCredentialResponse> {
        let mut client = self.client.clone();
        let resp = client.get_share_credential(req).await?;
        Ok(resp.into_inner())
    }
}
