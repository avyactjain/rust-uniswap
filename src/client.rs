use uniswap::uniswap_api_client::UniswapApiClient;
use uniswap::BalanceRequest;

pub mod uniswap{
   tonic::include_proto!("uniswap_api");
}

#[tokio::main]
async fn main()-> Result<(), Box<dyn std::error::Error>> {
    let mut client = UniswapApiClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(BalanceRequest{
        wallet_addr : "1234".to_string()
    });

    let response = client.get_wallet_balance(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
