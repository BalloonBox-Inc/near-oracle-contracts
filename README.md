<p align="center">
  <a href="https://near.org/">
    <img alt="Near" src="https://github.com/BalloonBox-Inc/near-oracle-contracts/blob/dev/images/inverted-primary-logo-bg.png" width="700" />
  </a>
</p>

# NEAR Smart Contracts
This repository contains the codebase of 2 independent smart contracts, hosted respectively in
- `contract-nft`: mint credit scores as NFTs
- `contract-storescore`: store credit scores on the NEAR blockchain

> Caught an error in our code or docs? Please, let us know [here](https://www.balloonbox.io/contact).
---

### :page_facing_up: Licences
This Git Repo is released under the Apache 2.0 open source license.
However, the codebase hosted under the directory `contract-nft` is regulated by the less rescrictive MIT Lincence. As such, we encourage developers to use such portion of the codebase entirely free of restrictions or limitations.

### :satellite: Purpose
The two smart contracts are completely independent from each other, but we decided to host them in the same Repo for simplicity. They are both written in Rust and are meant to be deployed to the Near Protocol blockchain. Our [BalloonBox](https://www.balloonbox.io/) team designed these contracts for [NearOracle](https://test.nearoracle.com/), a credit scoring dApp we built for the Near Foundation and its community.

### :octopus: Directory Structure
The structure of this Git Repo (directories and subdirectories) is as follow. The tree diagram disregards files of secondary importance and only displays the most important ones.

```bash
.
└─── 
    ├── contract-nft
    │   ├── res
    │   │   └── *.wasm                #the smart contract compiles to a .wasm file = the only file deployed to chain
    │   ├── src
    │   │   ├── approval.rs           #approve an account to transfer NFTs on your behalf
    │   │   ├── enumerate.rs          #contains view-only methods to view contract state
    │   │   ├── events.rs             #emit log events when NFTs are minted or transferred
    │   │   ├── internal.rs           #internal methods invoked within the contract itself
    │   │   ├── lib.rs                #core logic - most important file
    │   │   ├── metadata.rs           #metadata objects storing NFT info
    │   │   ├── mint.rs               #logic to mint NFTs
    │   │   ├── nft_core.rs           #logic to transfer NFTs
    │   │   ├── royalty.rs            #pay out perpetual royalties to someone
    │   │   ├── tests.rs              #unit tests of the codebase
    │   │   └── whitelist.rs          #grant someone permissions to invoke some methods
    │   ├── Cargo.lock                #auto generates from Cargo.toml
    │   ├── Cargo.toml                #the Rust manifest: declares all dependancies
    │   ├── README.md                 #docs on contract methods & their gas price
    │   ├── build.sh                  #shell commands to build and optimize your smart contract
    │   └── dev.md                    ##fork & deploy the contract from Near CLI
    ├── contract-storescore
    │   ├── res
    │   │   └── *.wasm                #the smart contract compiles to a .wasm file = the only file deployed to chain
    │   ├── src
    │   │   ├── lib.rs                #core logic - most important file
    │   │   └── whitelist.rs          #grant someone permissions to invoke some methods
    │   ├── Cargo.lock                #auto generates from Cargo.toml
    │   ├── Cargo.toml                #the Rust manifest: declares all dependancies
    │   ├── README.md                 #docs on contract methods & their gas price
    │   ├── build.sh                  #shell commands to build and optimize your smart contract
    │   └── dev.md                    #fork & deploy the contract from Near CLI
    ├── images
    ├── .gitignore
    ├── LICENSE
    ├── README.md
    ├── package.json
    └── rust-toolchain
```
