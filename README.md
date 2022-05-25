<p align="center">
  <a href="https://near.org/">
    <img alt="Near" src="https://github.com/BalloonBox-Inc/NEARoracle-Contract/blob/dev/images/inverted-primary-logo-bg.png" width="700" />
  </a>
</p>

---
# NEAR Oracle Contract 
### Requirements

node.js, npm (or yarn), Rust, and Wasm toolchain

##### Install Rust and Wasm toolchain

To [install Rust](https://doc.rust-lang.org/book/ch01-01-installation.html) on Linux or MacOS, use the following command:

```bash
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

Then, add the `wasm32-unknown-unknown` toolchain. This toolchain is required because the Rust contract that we will build will be compiled to [Wasm](https://webassembly.org/) (Web Assembly) to run on the NEAR blockchain.

```bash
rustup target add wasm32-unknown-unknown
```
 

#### Debugging 
###### Compile time errors
You must compile the smart contract before deploying in to blockchain. Compile the contract running the terminal command `./build.sh`. If compilation returns an error *unable to get packages from source* you might need to clear the cargo registry running `rm -rf /<userpathtocargoregistry>/.cargo/registry/`.


#### Navigating `near_sdk` Persistent Collections

> Note to all NEAR Rust developers: remember to choose your Rust objects based ont heir associated time complexity. Consult [this](https://docs.near.org/docs/concepts/data-storage#big-o-notation-1) table ranking object types in the `near_sdk' Rust collection by Big-O Notation.

#### Testing

```bash
cargo test -- --nocapture
cargo test --package near_oracle --  --nocapture      # Note: 'near_oracle' comes from Cargo.toml's 'name' key
```
