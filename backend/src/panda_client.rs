use tonic::transport::{Channel, Endpoint};

pub mod proto {
    tonic::include_proto!("lores.panda.v1");
}

use hex_literal::hex;
use proto::{
    panda_client::PandaClient as TonicPandaClient, OperationEvent, PublishRequest, SubscribeRequest,
};
use tonic::Streaming;

// Hardcoded dummy region_id and app_namespace for now.
const DUMMY_REGION_ID: [u8; 32] =
    hex!("003f1de60ac340ba64b73d3e97bd25f694c73ab178b52f246f8a05bcafcc1676");
const DUMMY_APP_NAMESPACE: &str = "chat-example:v1";

#[derive(Clone)]
pub struct PandaClient {
    inner: TonicPandaClient<Channel>,
}

impl PandaClient {
    /// Creates a client with a lazy channel — no connection is made until the
    /// first RPC call, so the backend starts cleanly even if the gRPC server
    /// is not yet available.
    pub fn new(addr: &str) -> Result<Self, tonic::transport::Error> {
        let channel = Endpoint::from_shared(addr.to_string())?.connect_lazy();
        Ok(Self {
            inner: TonicPandaClient::new(channel),
        })
    }

    pub async fn subscribe(&mut self) -> Result<Streaming<OperationEvent>, tonic::Status> {
        let request = SubscribeRequest {
            region_id: DUMMY_REGION_ID.to_vec(),
            app_namespace: DUMMY_APP_NAMESPACE.to_string(),
        };
        let response = self.inner.subscribe(request).await?;
        Ok(response.into_inner())
    }

    pub async fn publish(&mut self, payload: Vec<u8>) -> Result<(), tonic::Status> {
        let request = PublishRequest {
            region_id: DUMMY_REGION_ID.to_vec(),
            app_namespace: DUMMY_APP_NAMESPACE.to_string(),
            payload,
        };
        self.inner.publish(request).await?;
        Ok(())
    }
}
