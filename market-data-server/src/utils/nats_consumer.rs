use async_nats::jetstream::{
    self,
    consumer::{pull::Config, Consumer},
};
use shared::nats::get_jetstream_context;

const MARKET_DATA_SUBJECT: &str = "market-data";

pub(crate) async fn get_consumer() -> Consumer<Config> {
    log::info!("Connecting to NATS...");

    let jetstream = get_jetstream_context().await;

    log::info!("Connection Established...");

    // Get or create a stream
    let stream = jetstream
        .get_or_create_stream(jetstream::stream::Config {
            name: MARKET_DATA_SUBJECT.to_string(),
            max_messages: 10_000,
            ..Default::default()
        })
        .await;

    // Get or create a pull-based consumer
    stream
        .unwrap()
        .get_or_create_consumer(
            "consumer",
            async_nats::jetstream::consumer::pull::Config {
                durable_name: Some("consumer".to_string()),
                ..Default::default()
            },
        )
        .await
        .unwrap()
}
