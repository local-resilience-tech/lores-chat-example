use tonic::transport::{Channel, Endpoint};

pub mod proto {
    tonic::include_proto!("lores.panda.v1");
}

use proto::{
    panda_client::PandaClient as TonicPandaClient, ListRegionsRequest, OperationEvent,
    PublishRequest, SubscribeRequest,
};
use tonic::Streaming;

const APP_NAMESPACE: &str = "chat-example:v1";

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

    pub async fn subscribe(
        &mut self,
        region_id: [u8; 32],
    ) -> Result<Streaming<OperationEvent>, tonic::Status> {
        let request = SubscribeRequest {
            region_id: region_id.to_vec(),
            app_namespace: APP_NAMESPACE.to_string(),
        };
        let response = self.inner.subscribe(request).await?;
        Ok(response.into_inner())
    }

    pub async fn publish(
        &mut self,
        region_id: [u8; 32],
        payload: Vec<u8>,
    ) -> Result<(), tonic::Status> {
        let request = PublishRequest {
            region_id: region_id.to_vec(),
            app_namespace: APP_NAMESPACE.to_string(),
            payload,
        };
        self.inner.publish(request).await?;
        Ok(())
    }

    pub async fn list_regions(&mut self) -> Result<Vec<Vec<u8>>, tonic::Status> {
        let response = self.inner.list_regions(ListRegionsRequest {}).await?;
        Ok(response.into_inner().region_ids)
    }
}
