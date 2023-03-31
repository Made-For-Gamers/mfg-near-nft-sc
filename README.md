# mfg-near-nft-sc

Smart Contract Examples on NEAR

Ensure you have: npm install -g near-cli

# Wallet
To store your non-fungible tokens you'll need a NEAR Wallet. If you don't have one yet, you can create one easily by following these [instructions](https://wallet.testnet.near.org/create). 

# Building the contract
To build your contract run the following command in your terminal which builds your contract using Rust's cargo. 

## Build on Windows 
./scripts/build.bat

## Build on Linux 
./scripts/build.sh 

# Deploying the contract
This smart contract will be deployed to your NEAR account. Because NEAR allows the ability to upgrade contracts on the same account, initialization functions must be cleared.

### Log in to your newly created account with near-cli by running the following command in your terminal.

near login
 
#### Deploy Command:
near deploy [accountId] [wasmFile] [initFunction] [initArgs] [initGas] [initDeposit]

e.g. near deploy near deploy --wasmFile res/mysmartcontractbuildfile.wasm --accountId myaccount.testnet

near deploy --wasmFile res/simple_nft_mfg.wasm --initFunction new --initArgs "{\"owner_id\":\"asd.testnet\"}" --accountId asd.testnet

# Test the mint nft function (Note, you will need IPFS storage for reference of the NFT uri/media )
 
near call asd.testnet mint_nft "{\"token_id\":\"1\",\"receiver_id\":\"asd.testnet\",\"metadata\":{\"title\":\"NAME \",\"description\":\"One of a kind\",\"media\":\"https:\/\/asd.ipfs.nftstorage.link\"}}" --accountId asd.testnet

# View the ownership:

near view asd.testnet get_nft "{\"token_id\": \"1\"}"

# Transfer the ownership:

near call asd.testnet transfer_nft "{\"token_id\": \"1\", \"receiver_id\": \"asd.testnet\"}" --accountId asd.testnet