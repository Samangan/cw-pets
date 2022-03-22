#!/bin/bash

# Set env vars:
#source ./scripts/init-cliffnet-envvars.sh
#export NODE=(--node $RPC)
#export TXFLAG=($NODE --chain-id $CHAIN_ID --gas-prices 0.025upebble --gas auto --gas-adjustment 1.3)

cargo wasm
docker run --rm -v "$(pwd)":/code \
        --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
        --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
        cosmwasm/rust-optimizer:0.12.4

echo "Current version: ${CODE_ID}"
RES=$(wasmd tx wasm store artifacts/cw_pets.wasm --from testnet-wallet ${TXFLAG} -y --output json -b block)
CODE_ID=echo ${RES} | jq -r '.logs[0].events[-1].attributes[0].value'

echo "Deploying version: $CODE_ID"
CONTRACT=wasm1wsfhsnvwe2e82xnru0f0n9h08w6sh2v5n69smmzcr89dksdknq8qxjy2dp
wasmd tx wasm migrate $CONTRACT $CODE_ID "$MIGRATE_QUERY"  --from testnet-wallet $NODE $TXFLAG  -y

