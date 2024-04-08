use std::str::FromStr;

use crate::{
    config::UniswapAPIconfig,
    uniswap::{
        uniswap_api_server::UniswapApi, BalanceRequest, BalanceResponse, PriceRequest,
        PriceResponse,
    },
};
use rust_decimal::{prelude::FromPrimitive, Decimal};
use tonic::{Request, Response, Status};
use web3::{
    contract::{Contract, Error, Options},
    transports::WebSocket,
    types::{Address, H160, U256},
    Web3,
};

#[derive(Debug)]
pub struct UniswapService {
    pub config: UniswapAPIconfig,
    pub web3_client: Web3<WebSocket>,
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

    async fn get_price_from_pool(
        &self,
        request: Request<PriceRequest>,
    ) -> Result<Response<PriceResponse>, Status> {
        let req = request.into_inner();

        let address = Address::from_str(&req.contract_address);

        match address {
            Ok(h160_addr) => {
                let load_contract = Contract::from_json(
                    self.web3_client.eth(),
                    h160_addr,
                    include_bytes!("../abi/uniV3Pool.json"),
                );

                match load_contract {
                    Ok(contract) => {
                        let slot0_query: Result<(U256, i64, u128, u128, u128, u128, bool), Error> =
                            contract
                                .clone()
                                .query("slot0", (), None, Options::default(), None)
                                .await;

                        let token0_query: Result<Address, Error> = contract
                            .clone()
                            .query("token0", (), None, Options::default(), None)
                            .await;

                        let token1_query: Result<Address, Error> = contract
                            .clone()
                            .query("token1", (), None, Options::default(), None)
                            .await;

                        let reply: PriceResponse = match slot0_query {
                            Ok(slot0) => {
                                let sqrt_price_x_96 = slot0.0;

                                let two = U256::from(2);
                                let one_ninety_two = U256::from(192);
                                let ten = U256::from(10);
                                let tweleve = U256::from(12);

                                let token0 = token0_query.unwrap().to_string();
                                let token1 = token1_query.unwrap().to_string();

                                let x_float =
                                    sqrt_price_x_96.pow(two).to_string().parse::<f64>().unwrap();
                                let y_float =
                                    two.pow(one_ninety_two).to_string().parse::<f64>().unwrap();
                                let ten_12_float =
                                    ten.pow(tweleve).to_string().parse::<f64>().unwrap();

                                let price_decimal =
                                    Decimal::from_f64((x_float / y_float) * ten_12_float)
                                        .unwrap()
                                        .round_dp(4);

                                PriceResponse {
                                    successful: true,
                                    message: format!(
                                        "Price of 1 {token0} = {price_decimal} {token1}",
                                    ),
                                }
                            }
                            Err(e) => PriceResponse {
                                successful: false,
                                message: format!("Error : {e}"),
                            },
                        };

                        Ok(Response::new(reply))
                    }
                    Err(e) => Ok(Response::new(PriceResponse {
                        successful: false,
                        message: format!("Error : {e}"),
                    })),
                }
            }
            Err(e) => {
                return Ok(Response::new(PriceResponse {
                    successful: false,
                    message: format!("Error : {e}"),
                }));
            }
        }
    }
}
