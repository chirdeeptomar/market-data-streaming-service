pub(crate) mod config;

use shared::models::MarketData;
use std::env;
use stream::{wrappers::ReceiverStream, StreamExt};
use tokio::sync::mpsc;
use tokio_stream::{self as stream};
use tonic::{Request, Response, Status};

use crate::{
    proto::{market_data_service_server, MarketDataPayload, MarketDataRequest, MarketDataResponse},
    utils::{self, nats_consumer::get_consumer, Factory},
};

#[derive(Debug, Default)]
pub struct MarketDataServer {}

#[tonic::async_trait]
impl market_data_service_server::MarketDataService for MarketDataServer {
    /// Server streaming response type for the GetMarketDataStream method.
    type GetMarketDataStreamStream = ReceiverStream<Result<MarketDataPayload, Status>>;

    async fn get_market_data(
        &self,
        request: Request<MarketDataRequest>,
    ) -> Result<Response<MarketDataResponse>, Status> {
        let input = request.get_ref();
        log::info!(
            "Non-Streaming Request: Fetching Market data for: {} ",
            input.instrument,
        );
        let response = MarketDataResponse {
            response: vec![MarketDataPayload::new(input)],
        };
        Result::Ok(Response::new(response))
    }

    async fn get_market_data_stream(
        &self,
        request: Request<MarketDataRequest>,
    ) -> Result<Response<Self::GetMarketDataStreamStream>, Status> {
        let args = utils::args_parser::parse(env::args().collect());

        let input = request.get_ref();

        log::info!(
            "Streaming Request: Fetching Market data for: {} ",
            input.instrument
        );

        // Consume messages from the consumer
        let mut messages = get_consumer().await.messages().await.unwrap();

        // Create a channel to send streaming responses
        let (tx, rx) = mpsc::channel(128);

        tokio::spawn(async move {
            let deserializer =
                shared::serializer_factory::get_deserializer::<MarketData>(args.serializer_type);
            while let Some(item) = messages.next().await {
                // Unwrap the item from the Result
                let message = match item {
                    Ok(item) => item,
                    Err(err) => {
                        // Handle the error (e.g., log or break the loop)
                        log::error!("Error retrieving message: {:?}", err);
                        break;
                    }
                };

                // Deserialize the message into a MarketData struct
                let record: MarketData = match deserializer(&message) {
                    Ok(record) => record,
                    Err(err) => {
                        // Handle deserialization error (e.g., log or skip this message)
                        log::error!("Error deserializing message: {:?}", err.message);
                        continue; // Skip to the next message
                    }
                };

                let response = MarketDataPayload {
                    ask: record.ask,
                    bid: record.bid,
                    instrument: record.instrument,
                    volume: record.volume,
                    id: record.id.to_string(),
                    timestamp: record.timestamp.timestamp(),
                    asset_class: record.asset_class as i32,
                    side: record.side as i32,
                };

                match tx.send(Result::<_, Status>::Ok(response)).await {
                    Ok(_) => {
                        // item (server response) was queued to be send to client
                    }
                    Err(_item) => {
                        // output_stream was build from rx and both are dropped
                        break;
                    }
                }
            }
            log::warn!("\tclient disconnected");
        });

        let output_stream = ReceiverStream::new(rx);
        Ok(Response::new(output_stream))
    }
}
