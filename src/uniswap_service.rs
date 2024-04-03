use std::str::FromStr;

use crate::{
    config::UniswapAPIconfig,
    uniswap::{
        uniswap_api_server::UniswapApi, BalanceRequest, BalanceResponse, PriceRequest,
        PriceResponse,
    },
};
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

                        let token0: Result<Address, Error> = contract
                            .clone()
                            .query("token0", (), None, Options::default(), None)
                            .await; //WETH

                        let token1: Result<Address, Error> = contract
                            .clone()
                            .query("token1", (), None, Options::default(), None)
                            .await; //USDT

                        println!("Token0 is -> {:?}", token0);
                        println!("Token1 is -> {:?}", token1);

                        let reply: PriceResponse = match slot0_query {
                            Ok(slot0) => {
                                let sqrt_price_x_96 = slot0.0;

                                let x = sqrt_price_x_96.pow(U256::from(2));
                                let y = U256::from(2).pow(U256::from(192));
                                let ten_12: U256 = U256::from(10).pow(U256::from(12));

                                println!("x is -> {}", x);
                                println!("y is -> {}", y);
                                println!("ten_12 is -> {}", ten_12);


                                
                                //(x / y ) ** ten_12 is my answer

                                PriceResponse {

                                    successful: true,
                                    message: format!(
                                        "Price of ETH is : {:.1$} USDC",
                                        (x / y) * ten_12,
                                        4
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
