<p align="center">
  <a href="https://near.org/">
    <img alt="Near" src="https://github.com/irene-bbox/sc-near-crossword/blob/master/pic/near.png" width="250" />
  </a>
</p>


# Build & Deploy a NEAR Smart Contract
This guide teaches you how to compile, deploy, and interact with a Rust Smart Contract on NEAR testnet. For more comprehensive NEAR SDK documentation consult their official webpage [here](https://www.near-sdk.io/zero-to-hero/basics/set-up-skeleton)

---


## Create testnet account

Navigate to NEAR testnet [wallet](https://wallet.testnet.near.org) and click on 'Create Account'.


## Configure CLI

```bash
npm install -g near-cli                                                    # Install the NEAR CLI
near                                                                       # To see various possible commands run
near login                                                                 # Log into your NEAR testnet wallet
near keys <youraccountname>.testnet                                        # Visualize your keys running
```


## Set up Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh              # If you haven't installed Rust yet, install it now using rustup
rustup target add wasm32-unknown-unknown                                    # Add Wasm toolchain
```

Some -but not all- Smart Contracts (e.g., SCRT Network, NEAR Protocol, etc.) compile to WebAssembly (Wasm) and that's why we add the toolchain for Rust.
> :no_entry_sign: Note: NEAR never uses `cargo run`. Why? Because smart contracts are technically libraries and not binaries, so some blockchains, like NEAR, choose not to compile a contract via commonly used Rust commands, like `cargo run`.


## Prep smart contract

This guide assumes you already have a ready-to-deploy smart contract. If you haven't, download a simple smart contract template [here.](https://github.com/near-examples/rust-template)


## Deploy

There are 4 basic steps to deploy a Smart Contract on NEAR:
1. :wrench: build contract
2. :truck: create a sub-account (or delete and recreate it)
3. :clapper: deploy to subaccount
4. :surfer: interact 

`Cd` into the repo with the Rust codebase for your contract and run:

```bash
./build.sh                                                                                        # Build the contract
near create-account subaccountname.accountname.testnet --masterAccount accountname.testnet        # Create a subaccount
near state subaccountname.accountname.testnet                                                     # Check subaccount state
# now ensure you're in the directory that contains the 'res' directory, then run
near deploy subaccountname.accountname.testnet --wasmFile res/my_crossword.wasm                   # Deploy the contract
near state subaccountname.accountname.testnet                                                     # Check again state of subaccount
```

## Interact

There are few ways to interact with the contract, depending on whether you are calling on a method that is view-only or a method that changes the state of the contract. For view-only use the `near view` command:

```bash
near view subaccountname.accountname.testnet <name_of_viewing_function>
```

For state-handling use the `near call` command:

```bash
near call subaccountname.accountname.testnet <name_of_handling_function> '{"string": "Helloworld!"}' --accountId accountname.testnet
```
Notice that in the `near call` command we must include the `--accountId` flag. This is because changing the state of a contract cost a gas fee and thus we must specify which NEAR account we want to use to sign the transaction, and pay the gas fee.


## Reset account

Your best bet to start fresh with a smart contract is to first **delete** the subaccount (sending all remaining testnet Ⓝ to a recipient)  and then **create** the account again, in this way: 

```bash
near delete subaccountname.accountname.testnet accountname.testnet
near create-account subaccountname.accountname.testnet --masterAccount accountname.testnet
```
The first command deletes `subaccountname.accountname.testnet` and sends the rest of its NEAR to `accountname.testnet`.

## Unit test

Remember that a smart contract is technically a library as defined in the manifest file (which is the Cargo.toml). A consequence of writing a library in Rust is not having a 'main' function that runs, and that's why we use unit tests to ineract with the contract instead. Unit tests act as a helper during development. To execute your test run

```bash
cargo test                                              # w/o output 
cargo test -- --nocapture                               # this additional flag includes the test output
cargo test <name_of_test_function> -- --nocapture       # only run the specified test
```

