use std::str::FromStr;

use crate::{
    config::UniswapAPIconfig,
    uniswap::{uniswap_api_server::UniswapApi, BalanceRequest, BalanceResponse, EthPriceResponse},
};
use tonic::{Request, Response, Status};
use web3::{
    contract::{Contract, Error, Options},
    transports::WebSocket,
    types::H160,
    Web3,
};

#[derive(Debug)]
pub struct UniswapService {
    pub config: UniswapAPIconfig,
    pub web3_client: Web3<WebSocket>,
    pub eth_usdc_contract: Contract<WebSocket>,
}

#[tonic::async_trait]
impl UniswapApi for UniswapService {
    async fn get_wallet_balance(
        &self,
        request: Request<BalanceRequest>,
    ) -> Result<Response<BalanceResponse>, Status> {
        println!("Got a request : {:?}", request);
        let req: BalanceRequest = request.into_inner();

        let response = self
            .web3_client
            .eth()
            .balance(H160::from_str(&req.wallet_addr).unwrap(), None)
            .await;

        let reply = match response {
            Ok(balance) => BalanceResponse {
                successful: true,
                message: format!("Balance is : {} ETH", balance),
            },
            Err(e) => BalanceResponse {
                successful: false,
                message: format!("Error : {}", e),
            },
        };

        Ok(Response::new(reply))
    }

    async fn get_eth_price_from_pool(
        &self,
        _request: Request<()>,
    ) -> Result<Response<EthPriceResponse>, Status> {
        let slot0_query: Result<(u128, i64, u128, u128, u128, u128, bool), Error> = self
            .eth_usdc_contract
            .clone()
            .query("slot0", (), None, Options::default(), None)
            .await;

        let reply = match slot0_query {
            Ok(slot0) => {
                let sqrt_price_x_96 = slot0.0;

                let two: u128 = 2;
                let ten: u128 = 10;
                let q_96: u128 = two.pow(96);

                let p = (sqrt_price_x_96 as f64 / q_96 as f64).powf(2.0);

                let price_of_eth = ten.pow(12) as f64 / p;

                EthPriceResponse {
                    successful: true,
                    message: format!("Price of ETH is : {:.1$} USDC", price_of_eth, 4),
                }
            }
            Err(e) => EthPriceResponse {
                successful: false,
                message: format!("Error : {}", e),
            },
        };

        Ok(Response::new(reply))
    }
}
