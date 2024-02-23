pub mod proto {
    use tonic::include_file_descriptor_set;

    tonic::include_proto!("market_data_service");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        include_file_descriptor_set!("service_descriptor");
}

use rand::Rng;
use std::{net::ToSocketAddrs, time::Duration};
use stream::{wrappers::ReceiverStream, StreamExt};
use tokio::sync::mpsc;
use tokio_stream::{self as stream};
use tonic::{transport::Server, Request, Response, Status};

use proto::{
    market_data_service_server::MarketDataServiceServer, MarketDataPayload, MarketDataRequest,
    MarketDataResponse,
};

#[derive(Debug, Default)]
pub struct MarketDataServer {}

#[tonic::async_trait]
impl proto::market_data_service_server::MarketDataService for MarketDataServer {
    async fn get_market_data(
        &self,
        request: Request<MarketDataRequest>,
    ) -> Result<Response<MarketDataResponse>, Status> {
        let mut rng = rand::thread_rng();

        let input = request.get_ref();
        println!(
            "Non-Streaming Request: Fetching Market data for: {} ",
            input.instrument
        );
        let response = MarketDataResponse {
            response: vec![MarketDataPayload {
                instrument: input.instrument.to_string(),
                bid: rng.gen(),
                ask: rng.gen(),
                bid_size: None,
                ask_size: None,
                last_sale: None,
                last_size: None,
                quote_time: None,
                trade_time: None,
                exchange: None,
                volume: None,
            }],
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
        let mut rng = rand::thread_rng();

        // creating infinite stream with requested message
        let repeat = std::iter::repeat(MarketDataPayload {
            instrument: input.instrument.to_string(),
            bid: rng.gen(),
            ask: rng.gen(),
            bid_size: None,
            ask_size: None,
            last_sale: None,
            last_size: None,
            quote_time: None,
            trade_time: None,
            exchange: None,
            volume: None,
        });

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = MarketDataServer::default();

    let service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
        .build()?;

    Server::builder()
        .add_service(service)
        .add_service(MarketDataServiceServer::new(server))
        .serve("[::1]:50051".to_socket_addrs().unwrap().next().unwrap())
        .await
        .unwrap();

    Ok(())
}
