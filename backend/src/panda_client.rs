use tonic::transport::{Channel, Endpoint};

pub mod proto {
    tonic::include_proto!("lores.panda.v1");
}

use proto::{panda_client::PandaClient as TonicPandaClient, PublishRequest};

// Hardcoded dummy region_id (32 zero bytes) and app_namespace for now.
const DUMMY_REGION_ID: [u8; 32] = [0u8; 32];
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
