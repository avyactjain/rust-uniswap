use std::{net::SocketAddr, str::FromStr};

use tonic::transport::Server;
use uniswap::uniswap_api_server::UniswapApiServer;
use uniswap_service::UniswapService;

pub mod uniswap {
    tonic::include_proto!("uniswap_api");
}
mod config;
mod types;
mod uniswap_service;

use config::UniswapAPIconfig;
use web3::{contract::Contract, types::Address};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //load the config first
    let config: UniswapAPIconfig = UniswapAPIconfig::from_default_config_file();

    let web3_ws = web3::transports::WebSocket::new(&config.rpc_url).await?;
    let web3_client = web3::Web3::new(web3_ws);

    let addr: SocketAddr = config.listen_address.parse()?;

    println!("INFO! : rust-uniswap-server active on {}", addr.to_string());

    let uniswap_service = UniswapService {
        config,
        web3_client,
    };

    Server::builder()
        .add_service(UniswapApiServer::new(uniswap_service))
        .serve(addr)
        .await?;

    Ok(())
}
