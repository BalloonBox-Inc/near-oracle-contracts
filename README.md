# Rust Smart Contract Template

## Getting started

To get started with this template:

1. click the "Use this template" button to create a new repo based on this template
2. update line 2 of `Cargo.toml` with your project name
3. update line 4 of `Cargo.toml` with your project author names
4. setup the [Pre-requisits](https://github.com/near/near-sdk-rs#pre-requisites)
5. begin writing your smart contract in `src/lib.rs`
6. test the contract `cargo test`
7. build the contract `RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release`

**Get more info at:**

* [Rust Smart Contract Quick Start](https://docs.near.org/docs/develop/contracts/rust/intro)
* [Rust SDK Book](https://www.near-sdk.io/)
