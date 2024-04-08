
# GRPC API for uniswap v3 written in rust

This project is an api suite that will allow you to interact with uniswap v3 via grpc calls!

The project is in the initial stages and currently supports only ethereum mainnet with the following functionalities 

- GetWalletBalance : This will give you the ETH balance of a requested wallet address. 
- GetPriceFromPool : This will give you the current price of a tokens via the Pool's contract address.



## How to run? 
- Clone this repository on your machine. 
- Add your RPC url in config/local.json file. 
- Install BloomRPC/Postman on your machine. 
- In the project's directory, open a terminal and run the following command : 
```bash
 cargo run --bin rust-uniswap-server
```
- Open BloomRPC/Postman and load : rust-uniswap/proto/rust-uniswap.proto file. 
- Call the different functions listed in the proto file
  
## Pending Work
- Add Transaction capabilites to the project
- Remove Unwraps
- Adding tracing/logging module


<img src="demo.gif?raw=true" width="200px">
