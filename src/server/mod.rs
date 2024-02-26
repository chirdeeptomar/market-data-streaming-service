pub(crate) mod config;

use std::time::Duration;
use stream::{wrappers::ReceiverStream, StreamExt};
use tokio::sync::mpsc;
use tokio_stream::{self as stream};
use tonic::{Request, Response, Status};

use crate::{
    proto::{market_data_service_server, MarketDataPayload, MarketDataRequest, MarketDataResponse},
    utils::Factory,
};

#[derive(Debug, Default)]
pub struct MarketDataServer {}

#[tonic::async_trait]
impl market_data_service_server::MarketDataService for MarketDataServer {
    async fn get_market_data(
        &self,
        request: Request<MarketDataRequest>,
    ) -> Result<Response<MarketDataResponse>, Status> {
        let input = request.get_ref();
        println!(
            "Non-Streaming Request: Fetching Market data for: {} ",
            input.instrument
        );
        let response = MarketDataResponse {
            response: vec![MarketDataPayload::new(input)],
        };
        Result::Ok(Response::new(response))
    }

    /// Server streaming response type for the GetMarketDataStream method.
    type GetMarketDataStreamStream = ReceiverStream<Result<MarketDataPayload, Status>>;

    async fn get_market_data_stream(
        &self,
        request: Request<MarketDataRequest>,
    ) -> Result<Response<Self::GetMarketDataStreamStream>, Status> {
        let input = request.get_ref();
        println!(
            "Streaming Request: Fetching Market data for: {} ",
            input.instrument
        );

        // creating infinite stream with requested message
        let repeat = std::iter::repeat(MarketDataPayload::new(input));

        let mut stream = Box::pin(tokio_stream::iter(repeat).throttle(Duration::from_millis(200)));

        // spawn and channel are required if you want handle "disconnect" functionality
        // the `out_stream` will not be polled after client disconnect
        let (tx, rx) = mpsc::channel(128);
        tokio::spawn(async move {
            while let Some(item) = stream.next().await {
                match tx.send(Result::<_, Status>::Ok(item)).await {
                    Ok(_) => {
                        // item (server response) was queued to be send to client
                    }
                    Err(_item) => {
                        // output_stream was build from rx and both are dropped
                        break;
                    }
                }
            }
            println!("\tclient disconnected");
        });

        let output_stream = ReceiverStream::new(rx);
        Ok(Response::new(output_stream))
    }
}
