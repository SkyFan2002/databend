// Copyright 2021 Datafuse Labs.
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

use std::pin::Pin;
use std::thread::sleep;
use std::time::Duration;

use common_base::tokio;
use common_meta_types::protobuf::meta_server::Meta;
use common_meta_types::protobuf::meta_server::MetaServer;
use common_meta_types::protobuf::GetReply;
use common_meta_types::protobuf::GetRequest;
use common_meta_types::protobuf::HandshakeResponse;
use common_meta_types::protobuf::RaftReply;
use common_meta_types::protobuf::RaftRequest;
use futures::Stream;
use rand::Rng;
use tonic::transport::Server;
use tonic::Request;
use tonic::Response;
use tonic::Status;
use tonic::Streaming;

pub struct GrpcServiceForTestImpl {}

#[tonic::async_trait]
impl Meta for GrpcServiceForTestImpl {
    type HandshakeStream =
        Pin<Box<dyn Stream<Item = Result<HandshakeResponse, Status>> + Send + Sync + 'static>>;

    async fn handshake(
        &self,
        _request: Request<Streaming<common_meta_types::protobuf::HandshakeRequest>>,
    ) -> Result<Response<Self::HandshakeStream>, Status> {
        tokio::time::sleep(Duration::from_secs(2)).await;
        let output = futures::stream::once(async { Ok(HandshakeResponse::default()) });
        Ok(Response::new(Box::pin(output)))
    }

    async fn write_msg(
        &self,
        _request: Request<RaftRequest>,
    ) -> Result<Response<RaftReply>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
    }

    async fn read_msg(&self, _request: Request<GetRequest>) -> Result<Response<GetReply>, Status> {
        // for timeout test
        tokio::time::sleep(Duration::from_secs(60)).await;
        Err(Status::unimplemented("Not yet implemented"))
    }
}

pub fn start_grpc_server() -> String {
    let mut rng = rand::thread_rng();
    let port = rng.gen_range(10000..20000);
    let addr = format!("127.0.0.1:{}", port).parse().unwrap();
    let service = GrpcServiceForTestImpl {};

    let svc = MetaServer::new(service);

    tokio::spawn(async move {
        Server::builder()
            .add_service(svc)
            .serve(addr)
            .await
            .unwrap();
    });
    sleep(Duration::from_secs(1));
    addr.to_string()
}
