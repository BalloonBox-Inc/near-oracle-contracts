<p align="center">
  <a href="https://near.org/">
    <img alt="Near" src="https://github.com/BalloonBox-Inc/near-oracle-contracts/blob/dev/images/inverted-primary-logo-bg.png" width="700" />
  </a>
</p>

# Minting an NFT on NEAR Protocol :ringed_planet: :roller_skate: :kick_scooter:

In this guide, you'll learn how to mint an NFT on NEAR Protocol blockchain (testnet) via a Rust Smart Contract. This guide walks you through all steps for quick deployment of the contract, but doesn't walk explain the actual logic of the NFT-minting contract :nerd_face: :shipit: :bowtie:, if you want to learn how to write a smart contract o mint NFTs on near, then follow [this](https://docs.near.org/docs/tutorials/contracts/nfts/introduction) tutorial in the official NEAR documentation. 

---


## Create testnet account

If you don't own a NEAR wallet yet, create one. Navigate to NEAR testnet [wallet](https://wallet.testnet.near.org) and click on 'Create Account'.


## Configure CLI

```bash
npm install -g near-cli                                             # Install the NEAR CLI
near                                                                # To see various possible commands run
near login                                                          # Log into your NEAR testnet wallet
near keys <youraccountname>.testnet                                 # Visualize your keys running
```


## Set up Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh      # If you haven't installed Rust yet, install it now using rustup
rustup target add wasm32-unknown-unknown                            # Add Wasm toolchain
```

Some -but not all- Smart Contracts (e.g., SCRT Network, NEAR Protocol, etc.) compile to WebAssembly (Wasm) <br /> and that's why we add the toolchain for Rust :sheep:.

## Prep smart contract

This guide assumes you already have a ready-to-deploy smart contract. If you haven't, download from the official NEAR GitHub repo found [here](https://github.com/near-examples/nft-tutorial) a full-fledged smart contract to mint NFTs. Notice that this Repo has many branches. Each branch corresponds to the incremental spet in the smart contract development. Refer to the last branch if you want the most complete version of the contract. 

## Login Near wallet

To login to your Near wallet (testnet account), run in terminal the command below and follow the pop-up prompts
```bash
near login
```

## Deploy

Follow these 3 steps to deploy a Smart Contract on NEAR:
1. :gear: **build** the contract
2. :clapper: **deploy** the contract
3. :airplane: **initialize** the contract

Run in terminal,
```bash
export NFT_CONTRACT_ID=accountname.testnet                                  # Export path to your testnet account name
echo $NFT_CONTRACT_ID      
yarn build                                                                  # Build the contract

# now, ensure you are in the root folder and run
near deploy --wasmFile out/main.wasm $NFT_CONTRACT_ID                       # Deploy the contract
near call $NFT_CONTRACT_ID new_default_meta '{"owner_id": "'$NFT_CONTRACT_ID'"}' --accountId $NFT_CONTRACT_ID   # Initialize the contract
near call $NFT_CONTRACT_ID nft_mint '{"token_id": "nft1", "metadata": {"title": "May 4th", "description": "Star Wars pun", "media": "https://www.rd.com/wp-content/uploads/2020/04/GettyImages-1146900170.jpg"}, "receiver_id": "'$NFT_CONTRACT_ID'"}' --accountId $NFT_CONTRACT_ID --amount 0.1    # Mint the NFT
near view $NFT_CONTRACT_ID nft_metadata                                     # Call view functions
```
Congratulations :raised_hands: :tada: :partying_face: ! You've just minted an NFT on NEAR testnet. <br />
> * `accountname.testnet` is the name of your NEAR testnet account <br />
> * `NFT_CONTRACT_ID` is a path to a self-defined variable, i.e., your testnet account <br />
> * fields like `token_id`, `title`, `description`, etc. are customizable


## Redeploy (patch fixes)

Imagine you have an already-deployed smart contract. Assume you want to upgrade/change/add functionalities to such contract by altering its Rust code. The correct way to do so is by using *deployment patch fixes* (see official doc [here](https://docs.near.org/docs/tutorials/contracts/nfts/upgrade-contract)), namely code patches to a contract that had been previously deployed. To upgrade a contract follow the next steps:

* change the current code to add the desired functionality
* run in terminal from the root directory 
   
   ```bash
   yarn build && near deploy --wasmFile out/main.wasm --accountId $NFT_CONTRACT_ID
   ```
* this outputs a warning and will ask if you'd like to proceed. Simply type `y` and hit enter
* once the contract is redeployed, test that the state migrated correctly by running a simple a *view* function of your choice, e.g., `near view $NFT_CONTRACT_ID <my_function_name>`

> :no_entry: :radioactive: :warning: Patch fixes on NEAR require you to run the *yarn build && near deploy* commands **simultaneously**. If you try to execute these commands *consecutively* the operation will fail because the initial contract had already been deployed and the NEAR Runtime doesn't understand what to do with this contradictory request to deploy an already-deployed contract.  

## Interact

You're ready to interact with the smart contract. Use view calls to return viewing data from the smart contract.
```bash
near view $NFT_CONTRACT_ID json_token '{"token_id": "nft1"}'                  # View call 
```

---
## What does the smart contract actually do?
The functionalities of the smart contract are as follow:
 - mint a credit score as an NFT on Near blockchain
 - render the NFT among the __collectible__ tab of your Near wallet
 - query (for free) all the NFTs stored in the contract and returns them to you
 - pass in a user account and return all NFTs owned by that user
 - pass in an NFT token id and return who owns it
 - *log events whenever an NFT is minted or transferred (even in case of attempted and failed transfers)
 - *transfer an NFT from user A to user B
 - *grant/revoke permission to a user to transfer NFTs on your behalf
 - *pay out a perpetual royalty to some whitelisted addresses whenever an NFT is transferred
> *: these functionalities are beyond the scope of the Near grant, but we implemented them to easily scale up this project in the future


:hedgehog: :clapper: :mailbox_with_no_mail: 

We also constrained the smart contract logic to the following:
 - every user can mint at most X-many scores (X is an arbitrary integer)
 - the contract can mint at most Y-many scores (Y is an arbitrary integer)
 - every user can mint at most 1 score per month
 - you can not mint the same NFT (i.e., the same media uri) twice for the same user (no duplicates)
 - you can not mint multiple NFTs under the same token id (unique id required)
 - you can transfer an NFT from account A to account B iff you are either the NFT owner or you own an approval id
 - the `nft_mint` function can not be called from outside of the contract for security reasons
