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

near deploy --wasmFile res/simple_nft_mfg.wasm --initFunction new --initArgs "{\"owner_id\":\"mfg.testnet\"}" --accountId mfg.testnet

# Test the mint nft function (Note, you will need IPFS storage for reference of the NFT uri/media key)
 
near call mfg.testnet mint_nft "{\"token_id\":\"3\",\"receiver_id\":\"mfg.testnet\",\"metadata\":{\"title\":\"TheBulk \",\"description\":\"One of a kind\",\"media\":\"https:\/\/bafybeibjmmgynd4l5t7avhvzhv6chalyva3yiyazwjngjvkoifkzlyyxdi.ipfs.nftstorage.link\", \"custom_fields\":\" Custom Fields \"}}" --accountId mfg.testnet

# View the ownership:

near view mfg.testnet get_nft "{\"token_id\": \"1\"}"

# Transfer the ownership:

near call mfg.testnet transfer_nft "{\"token_id\": \"1\", \"receiver_id\": \"mfg.testnet\"}" --accountId mfg.testnet

near call mfg.testnet nft_mint "{\"token_id\":\"2\",\"receiver_id\":\"mfg.testnet\",\"token_metadata\":{\"title\":\"The Bulk\",\"description\":\"One of a Kind\",\"media\":\"https:\/\/bafybeibjmmgynd4l5t7avhvzhv6chalyva3yiyazwjngjvkoifkzlyyxdi.ipfs.nftstorage.link\/\",\"copies\":1}}" --accountId mfg.testnet --deposit 0.1

near call mfg.testnet nft_mint '{\"token_id\":\"2\",\"receiver_id\":\"mfg.testnet\",\"token_metadata\":{\"title\":\"TheBulk\",\"description\":\"The Bulk Avatar\",\"media\":\"https:\/\/bafybeibjmmgynd4l5t7avhvzhv6chalyva3yiyazwjngjvkoifkzlyyxdi.ipfs.dweb.link\",\"copies\":1}}'' --accountId mfg.testnet --deposit 0.1


near call mfg.testnet mint_nft "{"token_id": "3", "owner_id": "mfg.testnet", "title": "TheBulk", "description": "A unique token", "media": "https:\/\/bafybeibjmmgynd4l5t7avhvzhv6chalyva3yiyazwjngjvkoifkzlyyxdi.ipfs.dweb.link\"}" --accountId asd.testnet

//new 
near call mfg.testnet mint_nft "{\"token_id\":\"3\",\"receiver_id\":\"mfg.gg\",\"metadata\":{\"title\":\"TheBulk\",\"description\":\"The Real Dadbod\",\"media\":\"https:\/\/bafybeibjmmgynd4l5t7avhvzhv6chalyva3yiyazwjngjvkoifkzlyyxdi.ipfs.dweb.link\",\"media_hash\":null,\"copies\":1,\"expires_at\":null}}" --accountId mfg.testnet


  near call mfg.testnet mint_nft "{\"token_id\":\"3\",\"receiver_id\":\"mfg.testnet\",\"metadata\":{\"title\":\"TheBulk\",\"description\":\"The Real Dadbod\",\"media\":\"https:\/\/bafybeibjmmgynd4l5t7avhvzhv6chalyva3yiyazwjngjvkoifkzlyyxdi.ipfs.dweb.link\",\"media_hash\":null,\"copies\":1,\"expires_at\":null}}" --accountId mfg.testnet

near call <contract_account_id> mint_nft '{"token_id": "0", "receiver_id": "receiver_account_id", "metadata": { "title": "Some Art", "description": "My NFT media", "media": "https://bafkreiabag3ztnhe5pg7js4bj6sxuvkz3sdf76cjvcuqjoidvnfjz7vwrq.ipfs.dweb.link/", "media_hash": null, "copies": 1, "expires_at": null }}' --accountId <sender_account_id> --gas 300000000000000
