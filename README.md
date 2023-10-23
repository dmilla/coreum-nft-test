# Trying to deploy a Coreum Smart NFT for the sake of testing

The main contract src/contract.rs is implementing a very basic NFT that only the owner should be able to mint.

### Prerequisites
1. Install rust and cargo
2. Install cored binary
3. Install the required util jq
4. docker
5. network vars

### Creating a wallet
```
cored keys add wallet $COREUM_CHAIN_ID_ARGS
```
Then go to the faucet and fund your address: https://docs.coreum.dev/tools-ecosystem/faucet.html

### Compiling the contract
```
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer:0.13.0
```
This will create the artifacts folder with the WASM version of the contract


### Deploying the contract
```
RES=$(cored tx wasm store artifacts/nft_test.wasm \
    --from wallet --gas auto --gas-adjustment 1.3 -y -b block --output json $COREUM_NODE_ARGS $COREUM_CHAIN_ID_ARGS)
echo $RES    
CODE_ID=$(echo $RES | jq -r '.logs[0].events[-1].attributes[-1].value')
echo $CODE_ID
```
**Important** : we'll need the CODE_ID to be able to instantiate our contract later on!

### Instantiaing the contract
```
INIT="{\"name\": \"TestNFT\", \"symbol\": \"TNFT\"}"
cored tx wasm instantiate $CODE_ID "$INIT" --from wallet --label "test nft" -b block -y --no-admin $COREUM_NODE_ARGS $COREUM_CHAIN_ID_ARGS
```

```
cored q wasm list-contract-by-code $CODE_ID --output json $COREUM_NODE_ARGS $COREUM_CHAIN_ID_ARGS
CONTRACT_ADDRESS=$(cored q wasm list-contract-by-code $CODE_ID --output json $COREUM_NODE_ARGS $COREUM_CHAIN_ID_ARGS | jq -r '.contracts[-1]')
echo $CONTRACT_ADDRESS
```

### Interacting with the contract
```
MINT='{"mint_n_f_t": {"class_id": "tnft-testcore1f2n3ljrp0taugtz64y3w9vsnjl0zpaerhuaxk6hhq20tekvathqqr50370", "id": "test123", "data": "test"}}'
cored tx wasm execute $CONTRACT_ADDRESS "$MINT" --amount 100$COREUM_DENOM --from wallet -b block -y $COREUM_NODE_ARGS $COREUM_CHAIN_ID_ARGS

```
