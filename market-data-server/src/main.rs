mod proto;
mod server;
mod utils;

use proto::market_data_service_server::MarketDataServiceServer;
use server::config::{get_scheme_grpc, get_server_host, get_server_port};
use std::net::ToSocketAddrs;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address = format!("{}:{}", get_server_host(), get_server_port());

    let server = server::MarketDataServer::default();

    let service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
        .build()?;

    let grpc_server = Server::builder()
        .add_service(service)
        .add_service(MarketDataServiceServer::new(server))
        .serve(address.to_socket_addrs().unwrap().next().unwrap());

    log::info!("Server Listening on: {}://{}", get_scheme_grpc(), address);

    if let Err(e) = grpc_server.await {
        log::error!("server error: {}", e);
    }

    Ok(())
}
