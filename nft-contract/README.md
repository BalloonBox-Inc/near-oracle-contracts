<p align="center">
  <a href="https://near.org/">
    <img alt="Near" src="https://github.com/irene-bbox/sc-near-mintingNFT/images/near_oceanblue.png" 
    width="250" />
  </a>
</p>

# Minting an NFT on NEAR Protocol :ringed_planet: :roller_skate: :kick_scooter:
In this guide, you'll learn how to mint an NFT on NEAR Protocol blockchain (testnet) via a Rust Smart Contract. This guide walks you through all steps for quick deployment of the contract, but doesn't walk explain the actual logic of the NFT-minting contract :nerd_face: :shipit: :bowtie:, if you want to learn how to write a smart contract o mint NFTs on near, then follow [this](https://docs.near.org/docs/tutorials/contracts/nfts/introduction) tutorial in the official NEAR documentation. 
---


## Create testnet account

If you don't own a NEAR wallet yet, create one. Navigate to NEAR testnet [wallet](https://wallet.testnet.near.org) and click on 'Create Account' :handbag:.


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

Some -but not all- Smart Contracts (e.g., SCRT Network, NEAR Protocol, etc.) compile to WebAssembly (Wasm) and that's why we add the toolchain for Rust :sheep:.

## Prep smart contract

This guide assumes you already have a ready-to-deploy smart contract. If you haven't, download from the official NEAR GitHub repo found [here](https://github.com/near-examples/nft-tutorial) a full-fledged smart contract to mint NFTs. Notice that this Repo has many branches. Each branch corresponds to the incremental spet in the smart contract development. Refer to the last branch if you want the most complete version of the contract. 

## Login Near wallet

To login to your Near wallet (testnet account), run in terminal the command below and follow the pop-up prompts
```bash
near login
```

## Deploy

Follow these steps to deploy a Smart Contract on NEAR:
1. :gear: **build** the contract
2. :clapper: **deploy** the contract
3. :airplane: **initialize** the contract

Run in terminal,
```bash
export NFT_CONTRACT_ID=accountname.testnet
echo $NFT_CONTRACT_ID                                                                             # Export path to your testnet account name

yarn build                                                                                        # Build the contract

# now, ensure you are in the root folder and run
near deploy --wasmFile out/main.wasm $NFT_CONTRACT_ID                                             # Deploy the contract

near call $NFT_CONTRACT_ID new_default_meta '{"owner_id": "'$NFT_CONTRACT_ID'"}' --accountId $NFT_CONTRACT_ID   # Initialize the contract

near view $NFT_CONTRACT_ID nft_metadata                                                           # Call view functions

near call $NFT_CONTRACT_ID nft_mint '{"token_id": "nft1", "metadata": {"title": "May 4th", "description": "Star Wars pun", "media": "https://www.rd.com/wp-content/uploads/2020/04/GettyImages-1146900170.jpg"}, "receiver_id": "'$NFT_CONTRACT_ID'"}' --accountId $NFT_CONTRACT_ID --amount 0.1                    # Mint the NFT
```
Congratulations :raised_hands: :tada: :partying_face: ! You've just minted an NFT on NEAR testnet.
`<accountname.testnet>` is the name of your NEAR testnet account
`<NFT_CONTRACT_ID>` is a path to a self-defined variable, i.e., your testnet account

## Interact

You are now ready to interact with the smart contract. Do so, using view calls, which call on a function in the smart contract to return some viewing data.
```bash
near view $NFT_CONTRACT_ID nft_token '{"token_id": "nft1"}'                                         # View call 
```