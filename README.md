
# GRPC API for uniswap v3 written in rust

This project is an api suite that will allow you to interact with uniswap v3 via grpc calls!

The project is in the initial stages and currently supports only ethereum mainnet with the following functionalities 

- GetWalletBalance : This will give you the ETH balance of a requested wallet address. 
- GetEthPriceFromPool : This will give you the current price of ETH in USDC in uniswap V3. This connects directly to the ETH/USDC pool in uniswap. 




## How to run? 
- Clone this repository on your machine. 
- Add your RPC url in config/local.json file. 
- Install BloomRPC : https://appimage.github.io/BloomRPC/
- In the project's directory, open a terminal and run the following command : 
```bash
 cargo run --bin rust-uniswap-server
```
- Open BloomRPC and load : rust-uniswap/proto/rust-uniswap.proto file. 
- Call the different functions listed in the proto file
  
## Pending Work
- Add Transaction capabilites to the project