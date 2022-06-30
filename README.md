[![Basic](https://github.com/CodeMuhammed/cosmwasm_todo_app/actions/workflows/Basic.yml/badge.svg)](https://github.com/CodeMuhammed/cosmwasm_todo_app/actions/workflows/Basic.yml)


# CHIHUAHUAD: WORKING WITH SMART CONTRACT NOTES


## TEST ACCOUNTS: <chihuahuad keys list> : chihuahuad keys add <wallet_name> --recover

palingram: chihuahua1dpsvc7ml8mzkwtsdrcjfhgk6nmzsrhzj2auzkc
[april surprise correct arm radar stay broom lava actual thank pistol diet anger month lucky rely loyal loud correct exclude initial height category issue]

Wallet1: chihuahua15lhfr9vs7mzc6s4hmy9sgwpy4z8gvg9xjsuxpr
[desert people fossil siege wild hurt frozen person reflect gift range lizard crunch cry filter cake differ number ancient swear fiber knee march tray]

Wallet2: chihuahua1me3xey8rvgx3s4u83hctwzfea9wdsx4fxv9xsz
[goat term before goose close donor reunion robot catalog goat soul ivory horn trigger quarter item inmate hand gun consider else bag sound heavy]


## View chihuahuad config variables

open ~/.chihuahua/config/config.toml
chihuahuad config chain-id uni-2
rustup target list --installed
<https://github.com/oraichain/cw-ide-webview>


## Block explorer

wasmd: <https://block-explorer.malaga-420.cosmwasm.com/>

chihuahuad: <TODO>


## Source config env for use in the shell

source ~/.profile

export CHAIN_ID="chitestnet-1"

export RPC="tcp://65.108.126.34:26657"

export NODE=(--node $RPC)

export TXFLAG=($NODE --chain-id $CHAIN_ID --gas-prices 0.25stake --gas auto --gas-adjustment 1.3)


## wasmd reference varaibles

source <(curl -sSL <https://raw.githubusercontent.com/CosmWasm/testnets/master/malaga-420/defaults.env>)


## Query balance

chihuahuad query bank total $NODE
chihuahuad query bank total $NODE query bank balances $(chihuahuad keys show -a palingram) $NODE


## To run unit tests located in the .cargo/config file

RUST_BACKTRACE=1 cargo unit-test


## See the list of code uploaded to the testnet

chihuahuad query wasm list-code $NODE


## Generate a new cosm-wasm project from template

cargo install cargo-generate --features vendored-openssl
cargo generate --git <https://github.com/CosmWasm/cosmwasm-template.git> --name my-first-contract


## Compile the wasm contract with stable toolchain

rustup default stable
cargo wasm


## To compile a leaner version use

RUSTFLAGS='-C link-arg=-s' cargo wasm


## To compile an optimized build with a docker image

docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer:0.12.6


cargo install cargo-run-script

If this optimized script is already added to the [package.metadata.scripts] section of the cargo.toml

cargo run-script optimize


## STORE THE SMART CONTRACT TO THE BLOCKCHAIN

export RES=$(chihuahuad tx wasm store artifacts/my_first_contract.wasm --from palingram $TXFLAG -y --output json -b block)

echo $RES

export CODE_ID=$(echo $RES | jq -r '.logs[0].events[-1].attributes[0].value')

echo $CODE_ID


## Get a list of contracts instantiated for $CODE_ID

chihuahuad query wasm list-contract-by-code $CODE_ID $NODE --output json


## Verify if the binary stored on-chain for $CODE_ID matches with the local build

chihuahuad query wasm code $CODE_ID $NODE artifacts/existing_binary.wasm
diff artifacts/cw_nameservice.wasm artifacts/existing_binary.wasm


## INSTANTIATING THE SMART CONTRACT FOR $CODE_ID

### Prepare the json.stringigied message payload

export INIT='{}'


### Instantiate the contract

chihuahuad tx wasm instantiate $CODE_ID "$INIT" --from palingram --label "BURN TEST CONTRACT" $TXFLAG -y --no-admin


### Get the latest contract instantiated for contract with $CODE_ID

export CONTRACT=$(wasmd query wasm list-contract-by-code $CODE_ID $NODE --output json | jq -r '.contracts[-1]')
echo $CONTRACT


### Check the contract details

chihuahuad query wasm contract $CONTRACT $NODE


### Check the contract balance

chihuahuad query bank balances $CONTRACT $NODE


### query the entire contract state

chihuahuad query wasm contract-state all $CONTRACT $NODE


### query the data for a storage key in the contract-state directly

chihuahuad query wasm contract-state raw $CONTRACT 636F6E74726163745F696E666F $NODE  --output "json" | jq -r '.data' | base64 -d


## INTERACTING WITH  $CONTRACT

### Calling execute methods

export E_PAYLOAD='{"delete_entry":{"id":1}}'
chihuahuad tx wasm execute $CONTRACT "$E_PAYLOAD" --amount 1000000000stake --from palingram $NODE $TXFLAG -y


### calling query methods

export Q_PAYLOAD='{"query_list":{}}'
chihuahuad query wasm contract-state smart $CONTRACT "$Q_PAYLOAD" $NODE --output json


## Run the following command to start the node REPL this is complete with cosmos sdk interactions

npx @cosmjs/cli@^0.28.1 --init <https://raw.githubusercontent.com/InterWasm/cw-plus-helpers/main/base.ts> --init <https://raw.githubusercontent.com/InterWasm/cw-plus-helpers/main/cw20-base.ts>
<https://docs.cosmwasm.com/docs/1.0/getting-started/interact-with-contract>
<https://www.npmjs.com/package/@cosmjs/cli>


## Useful links
<https://github.com/cosmos/cosmjs/blob/main/packages/cli/examples/local_faucet.ts>
