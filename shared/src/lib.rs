pub mod enums;
pub mod models;
pub mod serializer;
pub mod serializer_factory;

pub mod nats {
    use async_nats::jetstream::{self, Context};

    pub async fn get_jetstream_context() -> Context {
        let nats_url = std::env::var("NATS_URL").unwrap_or_else(|_| "0.0.0.0:4222".to_string());
        let client = async_nats::connect(nats_url).await.unwrap();
        jetstream::new(client)
    }
}
