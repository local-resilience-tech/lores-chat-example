use tonic::transport::Channel;

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
    pub async fn connect(addr: String) -> Result<Self, tonic::transport::Error> {
        let inner = TonicPandaClient::connect(addr).await?;
        Ok(Self { inner })
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
