<p align="center">
  <a href="https://near.org/">
    <img alt="Near" src="bit.ly/3NLYZZA" width="250" />
  </a>
</p>


# Build & Deploy a NEAR Smart Contract
This guide teaches you how to compile, deploy, and interact with a Rust Smart Contract on NEAR testnet. For more comprehensive NEAR SDK documentation consult their official webpage [here](https://www.near-sdk.io/zero-to-hero/basics/set-up-skeleton)

---


## Quick Start


## Configure CLI

```bash
npm install -g near-cli                   # Install the NEAR CLI
near                                      # To see various possible commands run
near login                                # Log into your NEAR testnet wallet
near keys <youraccountname>.testnet       # Visualize your keys running
```


## Set up Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh              # If you haven't installed Rust yet, install it now using rustup
rustup target add wasm32-unknown-unknown                                    # Add Wasm toolchain
```

Some -but not all- Smart Contracts (e.g., SCRT Network, NEAR Protocol, etc.) compile to WebAssembly (Wasm) and that's why we add the toolchain for Rust.
> :no_entry_sign: Note: NEAR never uses `cargo run`. Why? Because smart contracts are technically libraries and not binaries, so some blockchains, like NEAR choose not to compile a contract via commonly used Rust commands, like `cargo run`.


