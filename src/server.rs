use std::{net::SocketAddr, str::FromStr};

use tonic::transport::Server;
use uniswap::uniswap_api_server::UniswapApiServer;
use uniswap_service::UniswapService;

pub mod uniswap {
    tonic::include_proto!("uniswap_api");
}
mod config;
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

    let contract = Contract::from_json(
        web3_client.eth(),
        Address::from_str("0x88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640").unwrap(),
        include_bytes!("/Users/avyactjain/defi/rust-uniswap/abi/uni_v3_eth_usdc_pool.json"),
    )
    .expect("Error : Unable to load the ETH/USDC pool smart-contract.");

    let uniswap_service = UniswapService {
        config,
        web3_client,
        eth_usdc_contract: contract,
    };

    Server::builder()
        .add_service(UniswapApiServer::new(uniswap_service))
        .serve(addr)
        .await?;

    Ok(())
}
