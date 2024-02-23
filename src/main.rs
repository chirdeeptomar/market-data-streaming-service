pub mod proto {
    tonic::include_proto!("market_data_service");
}
use rand::Rng;
use std::net::ToSocketAddrs;
use stream::wrappers::ReceiverStream;
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
        println!("Serving Streaming request");
        todo!()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = MarketDataServer::default();

    Server::builder()
        .add_service(MarketDataServiceServer::new(server))
        .serve("[::1]:50051".to_socket_addrs().unwrap().next().unwrap())
        .await
        .unwrap();

    Ok(())
}
