<p align="center">
  <a href="https://near.org/">
    <img alt="NearMonotoneWhite" src="https://github.com/BalloonBox-Inc/near-oracle-contracts/blob/dev/images/monotone-black.png" width="550" />
  </a>
</p>


# Minting NFTs on NEAR Protocol  :turtle: :fairy: :sunglasses:

In this guide, you'll learn how to utilize the codebase of a Rust smart contract to mint NFT on NEAR Protocol. All it takes is 5 simple steps. Notice that these instructions are for NEAR *testnet* only.

---
## Requirements 
The requirements for this project are: node.js, npm (or yarn), Rust, and Wasm toolchain.
If you haven't set up all requirements yet, do so following [these](https://github.com/BalloonBox-Inc/near-oracle-contracts/blob/main/contract-storescore/dev.md) instructions. 

## Deploy
Deploy the smart contract in 5 steps:
1. :chains: **clone** the codebase
2. :hammer_and_wrench: **build** the contract (auto compile)
3. :clapper: **deploy** the contract
4. :airplane: **initialize** the contract
5. :golf: **interact** with the contract

Run in terminal,
```bash
git clone https://github.com/BalloonBox-Inc/near-oracle-contracts.git   <path_to_your_local_dir> # clone this Git Repo locally
export A1=bruno.testnet                                                      # Export path to your testnet account names
export A2=benji.testnet
echo $A1      

cd contract-nft
yarn build && near deploy --wasmFile res/nftmint.wasm --accountId $A1        # Deploy the contract
near call $A1 new_default_meta '{"owner_id": "'$NA1'"}' --accountId $A1      # Initialize the contract
near call $A1 nft_mint '{"token_id": "001", "metadata": {"title": "SpaceN", "description": "SpaceN: Falcon Heavy", "media": "https://c.tenor.com/RaotAGr2LeYAAAAC/near-near-blockchain.gif"}, "receiver_id": "'$A1'"}' --accountId $A1 --amount 0.1   # Mint the NFT

# remember: only whitelisted users can call nft_mint(). So, whitelist if needed
near call $A1 add_to_whitelist '{"account_id":"'$A2'"}' --accountId $A1       # Now benji.testnet can call mint_nft()                                          
```
You can now interact with the contract, calling viewing methods
```bash
 near view $A1 nft_metadata                                                   # Read contract state
 near view $A1 nft_total_supply --accountId $A1                               # Total count of NFTs in the contract
 near view $A1 nft_tokens '{"from_index":"0"}' --accountId $A1                # List of NFT metadata in the contract
 near view $A1 nft_supply_for_owner '{"account_id": "'$A1'"}' --accountId $A1 # NFT count for an owner
 near view $A1 nft_tokens_for_owner '{"account_id": "'$A1'", "from_index":"0"}' --accountId $A1 # List of NFTs for an owner
 near view $A1 json_token '{"token_id": "001"}'                               # Return metadata for passed in token
```

You can also pay out royalties and grant approvals for users to transfer NFTs on your behalf.
```bash
near call $A1 nft_approve '{"token_id": "001", "account_id": "'$A2'"}' --accountId $A1 --deposit 0.1
near call $A1 nft_transfer '{"receiver_id":"'$A1'", "token_id":"001", "approval_id":0}' --accountId $A1 --depositYocto 1
near call $A1 nft_transfer_call '{"receiver_id": "'$A2'", "token_id": "001", "msg": "NFT Transfer"}' --accountId $A1 --depositYocto 1 --gas 200000000000000
near call $A1 nft_mint '{"token_id": "002", "metadata": {"title": "Planet", "description": "Near planet", "media": "https://cryptosrus.com/wp-content/uploads/2021/12/NEAR-Protocol.png"}, "receiver_id": "'$A2'", "perpetual_royalties":{"spensa.testnet":500, "doomslug.testnet":800}}' --accountId $A1 --amount 0.1
near view $A1 nft_payout '{"token_id": "001", "balance": "100", "max_len_payout": 100}' # Calculate payout
```
Congratulations :raised_hands: :tada: :partying_face: ! You've just minted an NFT on NEAR testnet. <br />
> * `bruno.testnet` is the name of your NEAR testnet account <br />
> * `A1` is a path to a self-defined variable, i.e., your testnet account <br />
> * fields like `token_id`, `title`, `description`, etc. are customizable


## Redeploy (patch fixes)

Imagine you have an already-deployed smart contract. Assume you want to upgrade/change/add functionalities to such contract by altering its Rust code. The correct way to do so is by using *deployment patch fixes* (see official doc [here](https://www.near-sdk.io/upgrading/prototyping) and [here](https://docs.near.org/develop/upgrade-and-lock)), namely code patches to a contract that had been previously deployed. To upgrade a contract follow the next steps:

* change the current code to add the desired functionality
* run in terminal from the root directory 
   
   ```bash
   yarn build && near deploy --wasmFile out/main.wasm --accountId $A1            #customize the output path if needed, e.g., res/*.wasm
   ```
* this outputs a warning and will ask if you'd like to proceed. Simply type `y` and hit enter
* once the contract is redeployed, test that the state migrated correctly by running a simple *view* function of your choice, e.g., `near view $A1 <my_function_name>`

> :no_entry: :radioactive: :warning: Patch fixes on NEAR require you to run the *yarn build && near deploy* commands **simultaneously**. If you try to execute these commands *consecutively* the operation will fail because the initial contract had already been deployed and the NEAR Runtime doesn't understand what to do with this contradictory request to deploy an already-deployed contract. It's an excellent practice to deploy your smart contract in a subaccout in the very first place (e.g., `v1.bbox.testnet`). Once you want to upgrade your contract, redeploy the new codebase to a fresh subaccount (e.g., `v2.bbox.testnet` and so on).