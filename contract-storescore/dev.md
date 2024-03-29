<p align="center">
  <a href="https://near.org/">
    <img alt="NearMonotoneWhite" src="https://github.com/BalloonBox-Inc/near-oracle-contracts/blob/dev/images/monotone-black.png" width="550" />
  </a>
</p>


# Storing Scores on NEAR Protocol	:eyes:
This is a smart contract in Rust that runs on the NEAR Protocol blockchain. The contract runs in the backend of NearOracle, a credit scoring oracle built on NEAR. The oracle returns a numerical score affirming users' credibility and trustworthiness in the web3 space. The dApp was designed with one specific use case in mind: unsecured P2P lending, which is facilitating lending and borrowing of crypto loans. The dApp works as follows:

- it acquires user's financial data by integrating with either or three validators ([Plaid](https://dashboard.plaid.com/overview), [Coinbase](https://developers.coinbase.com/), [MetaMask](https://metamask.io/))
- it runs an algorithm on given data to compute a score representing the financial health of a user
- it writes the score to the NEAR Protocol blockchain via a smart contract built using the Rust `NEAR SDK`

The complete source code of the algorithm is stored in [this](https://github.com/BalloonBox-Inc/near-oracle-algorithm) other Git Repo. The rest of these docs are written with the developer's experience in mind. Follow the guideline to clone the repo and deploy the contract yourself under a new near account of your choice. Start by setting up the following prerequisites.


---
### 1. :moneybag: NEAR wallet 
[Create](https://wallet.near.org/) a NEAR wallet following the official NEAR [docs](https://docs.near.org/docs/develop/basics/create-account). Once the account is running, you can interact with it:
```bash
export A1=parent.testnet
export A2=child.parent.testnet

near login                                       # log into your wallet
near keys $A1                                    # query and see the keys associated with your account
near state $A1                                   # view the state of your account
near create-account $A2 --masterAccount $A1      # create a sub-account from a main account
near delete $A2 $A1                              # delete an account and transfer leftover funds to a beneficiary master account
near send sender.testnet receiver.testnet 1      # send 1 NEAR to receiver.testnet from sender.testnet
```
> :bulb: note: replace `parent` and `child` in the above commands with the names of your main and sub-account, e.g., `michael.testnet`

* Why do we need sub-accounts? To simulate the interaction of multiple users with the same smart contract
* Why would we ever want to delete an account? Altering the state of a contract after that contract got deployed can be tricky, so in some cases, it's best to start fresh, delete the old account, create it again, and deploy the updated contract from the new account.


### 2. :keyboard: Configure CLI
Install the NEAR CLI.
```bash
npm install -g near-cli                                             # Install the NEAR CLI
near                                                                # To see various possible commands run
near login                                                          # Log into your NEAR testnet wallet
near keys <youraccountname>.testnet                                 # Visualize your keys running
```

### 3. :gear: Set up Rust
Project requirements: node.js, npm (or yarn), [Rust](https://doc.rust-lang.org/book/ch01-01-installation.html), and Wasm toolchain.
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh      # If you haven't installed Rust yet, install it now using rustup
rustup target add wasm32-unknown-unknown                            # Add Wasm toolchain
```

The Rust toolchain is required because the contract compiles to [Wasm](https://webassembly.org/) (Web Assembly) to run on the NEAR blockchain. In fact, some -but not all- Smart Contracts (e.g., SCRT Network, Internet Computer, NEAR Protocol, etc.) compile the code to a `*.wasm` file.


### 4. :hammer_and_pick: Testing 

To run the unit tests on your contract, run from terminal
```bash
cargo test
cargo test --package storescore --  --nocapture      # Note: 'storescore' comes from Cargo.toml's 'name' key
```
> :no_entry_sign: Beware: you can only test the `dev` branch of this project. If you try and execute tests in the `main` branch they will break, because `main` implements a smart contract that is constrained to storing/minting at most 1 score/month, whereas unit tests require the same user to mint multiple scores consecutively (without waiting for a month). So, ensure you run `cargo test` from the `dev` branch. If your code passed the tests, you are now ready to deploy it on testnet.


### 5. :zap: Compile, Deploy, Initialize the Contract 
The life cycle of a NEAR smart contract is the following: compile, deploy, initialize, interact. 
  * compile your Rust code into a wasm file - locally in your machine - 
  * deploy the `.wasm` file to the NEAR blockchain
  * initialize the on-chain contract invoking a (default or custom) initialization function
  * interact with the contract by sending on-chain function calls (state handling operations which cost a gas fee) or view calls (view-only operations which are costless)

Deploy from terminal with these commands. You must be in the directory containing the *Cargo.toml* file and the *scr* and *res* folders.
```bash
export A1=parent.testnet
export PATH="$HOME/.cargo/bin:$PATH"                                       # (optional) export path to cargo files
./build.sh                                                                 # compile 
yarn build && near deploy --wasmFile res/storescore.wasm --accountId $A3   # deploy
near call $A1 new '{"owner_id": "'$A1'"}' --accountId $A1                  # initialize
```
> :bulb: note: replace `parent.testnet` with the actual name of your testnet account


### 6. :dart: Interact with the Contract 
Now we're ready to interact!

To store a score run
```bash
near call $A1 store_score '{"score": 700, "description": "Congrats! 700 points"}' --accountId $A1
```

To query a user's score history run
```bash
near call $A1 query_score_history '{"account_id": "myname.testnet"}' --accountId $A1
```

To check whether a user has a score record run
```bash
near call $A1 user_exist '{"account_id": "myname.testnet"}' --accountId $A1
```

To query contract state, run
```bash
near view $A1 read_state
```

> :warning: :radioactive: :stop_sign: owner, signer, predecessors: a user can have multiple roles relative to a contract:
> * the `owner` is the user account that owns the smart contract;
> * the `signer` is the user that signed the last transaction or action relating to the contract;
> * the `predecessor` is the user that interacted with the contract last (i.e., most recently).
> example: the very *first* user that appears in the terminal commands listed above is the owner of the contract we are calling. The user appearing right after the `--accountId` flag is the signer of the transaction.

---
### :weight_lifting_woman: Using `near_sdk` Persistent Collections

> Note to NEAR Rust developers: remember to choose your Rust objects based on their associated time complexity. Consult [this](https://docs.near.org/concepts/storage/data-storage) table ranking object types in the `near_sdk' Rust collection by Big-O Notation.
> Remember that all objects (structs, enums, etc.) which 'live' on-chain, should preferably be objects in the NEAR persistent collections, whereas objects that 'live' off-chain *must* be Rust std collections or Rust objects of some sort. near_sdk objects only exist on-chain and can't be rendered off-chain.

### :racing_car: Gas Fees

> The smart contract contains both some payable and some gasless methods. You can easily tell apart a payable method because of the macro `#[paybale]` above the method declaration. Payable methods cost a discretionary amount of gas -established by the NEAR Protocol- and you must *always* invoke them through a *function call*. All other methods that don't alter the contract state are gasless and are to be invoked through a *function view*. Both gas and gasless methods may or may not require some function parameters to be parsed. 

> What is gas actually charging for? A few things
> - data sorage on blockchain
> - common and complex [actions](https://docs.near.org/docs/concepts/gas#the-cost-of-common-actions)
> - [function calls](https://docs.near.org/docs/concepts/gas#function-calls) 


### :beetle: Debugging
###### Compile time errors
You must compile the smart contract before deploying it to blockchain. Compile the contract running the terminal command `./build.sh`. If compilation returns an error *unable to get packages from source* you might need to clear the cargo registry running `rm -rf /<userpathtocargoregistry>/.cargo/registry/`.


###### Upgrading Contracts

To upgrade contracts you need to first understand the difference between the code and state of a smart contract (official docs [here](https://www.near-sdk.io/upgrading/prototyping)). When a contract is deployed on top of an existing contract, the only thing that changes is the code, while the state remains the same causing developer issues.

When your contract is executed, the NEAR Runtime reads the serialized state from the disk and attempts to load it using the current contract code. When your code changes but the serialized state stays the same, it can't figure out how to do this. You need to strategically upgrade your contracts and make sure that the runtime will be able to read your current state with the new contract code. What's the best practice to upgrade a contract?

* If you're still in the R&D phase and want to deploy your prototype contract locally or on testnet, then you should delete all previous contract state by either:
  1) running in terminal `rm -rf neardev && near dev-deploy`
  2) or deleting and recreating the near wallet account

* If you're ready to deploy a stable contract in production, you'll want to migrate the contract state following carefully these [production strategies](https://www.near-sdk.io/upgrading/production-basics). Once your contract graduates to community-governed mode, you'll have to upgrade the code via a [DAO vote](https://www.near-sdk.io/upgrading/via-dao-vote).