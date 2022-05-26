<p align="center">
  <a href="https://near.org/">
    <img alt="Near" src="https://github.com/BalloonBox-Inc/NEARoracle-Contract/blob/dev/images/inverted-primary-logo-bg.png" width="700" />
  </a>
</p>

---
# NEAR Oracle Contract 
## 	:eyes: At a Glance
This is a smart contract in Rust that runs on the NEAR Protocol blockchain. The contract runs in the backend of the NEARoracle, a DApp for credit scoring built on NEAR. The oracle returns a numerical score affirming users' credibility and trustworthiness in the web3 space. The DApp was designed with one specific use case in mind: unsecured P2P lending, which is facilitating lending and borrowing of crypto loans. The DApp works as follows:

- it acquires user's financial data by integrating with either or three validators ([Plaid](https://dashboard.plaid.com/overview), [Coinbase](https://developers.coinbase.com/), [Near](https://wallet.near.org/))
- it runs an algorithm on given data to compute a score representing the financial health of a user
- it writes the score to the NEAR Protocol blockchain via a Wasm smart contract build using the Rust `NEAR SDK`

The complete source code of the algorithm is stored [here](https://github.com/BalloonBox-Inc/NEARoracle-Oracle).

## Fork or Execute Locally
The rest of these docs are written with the developer's experience in mind. Follow the guideline to execute the contract yourself. This smart contract was already deployed under the `bbox.testnet` NEAR account. You can choose to either: (a) interact with the already-deployed contract; (b) deploy the contract yourself under a new account of your choice. If either case, the following is required. 

### 1. :hammer_and_wrench: Requirements 

node.js, npm (or yarn), Rust, and Wasm toolchain

##### Install Rust and Wasm toolchain

To [install Rust](https://doc.rust-lang.org/book/ch01-01-installation.html) on Linux or macOS, use the following command:

```bash
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

Then, add the `wasm32-unknown-unknown` toolchain. This toolchain is required because the Rust contract compiles to [Wasm](https://webassembly.org/) (Web Assembly) to run on the NEAR blockchain.

```bash
rustup target add wasm32-unknown-unknown
```
 
### 2. :luggage: Create a NEAR wallet 
[Create](https://wallet.near.org/) a NEAR wallet following the official NEAR [docs](https://docs.near.org/docs/develop/basics/create-account). Once the account is running, you can start interacting with it:
```bash
near login                                       # log into your wallet
near keys main.testnet                           # query and see the keys associated with your account
near state main.testnet                          # view the state of your account
near create-account sub.main.testnet --masterAccount main.testnet
near delete sub.main.testnet main.testnet        # delete an account and transfer leftover funds to a beneficiary account -the one mentioned last-
```
> :bulb: note: replace `main` and `sub` in the above commands with the names of your main and sub-account, e.g., `doomslug.testnet`

* Why do we need subaccounts? To simulate the interaction of multiple users with the same smart contract
* Why would we ever want to delete an account? Altering the state of a contract after that contract got deployed can be tricky, so in some cases, it's best to start fresh, delete the old account, create it again, and deploy the updated contract from the new account

### 3. :toolbox: Testing 

To run the unit tests on your contract, run from terminal
```bash
cargo test
cargo test --package near_oracle --  --nocapture      # Note: 'near_oracle' comes from Cargo.toml's 'name' key
```
If your code passed the tests, you are now ready to deploy it on testnet.

### 4. :zap: Compile, Deploy, Initialize the Contract 
The life cycle of a NEAR smart contract is the following: compile, deploy, initialize, interact. 
  * compile your Rust code into a wasm file - locally in your machine - 
  * deploy the wasm file to the NEAR blockchain
  * initialize the on-chain contract invoking a (default or custom) initialization function
  * interact with the contract by sending on-chain function calls (state handling operations which cost a gas fee) or view calls (view-only operations which are costless)

Here are the commands to run the contract from terminal. You must be in the directory containing the *Cargo.toml* file and the *scr* and *res* folders.
```bash
export PATH="$HOME/.cargo/bin:$PATH"                                       # (optional) export path to cargo files
./build.sh                                                                 # compile 
near deploy storescore.bbox.testnet --wasmFile res/near_oracle.wasm        # deploy
near call myname.testnet new '{"owner_id": "myname.testnet"}' --accountId myname.testnet # initialize
```
> :bulb: note: replace `myname.testnet` with the actual name of your testnet account


### 5. :dart: Interact with the Contract 
Now we're ready to interact!

To store a score run
```bash
near call myname.testnet store_score '{"score": 650, "description": "Congrats! 300 points"}' --accountId myname.testnet
```

To query a user's score history run
```bash
near call myname.testnet query_score_history '{"account_id": "myname.testnet"}' --accountId myname.testnet
```

To check whether a user has a score record run
```bash
near call myname.testnet user_exist '{"account_id": "myname.testnet"}' --accountId myname.testnet
```

To query contract state run
```bash
near view myname.testnet read_state
```

> :warning: :radioactive: :stop_sign: owner, signer, predecessors: a user can have multiple roles relative to a contract it interacted with:
> * the `owner` if the user account that deployed and initialized the contract;
> * the `signer` is the user that signed the last transaction or action relating to the contract;
> * the `predecessor` is the user that interacted with the contract last (i.e., most recently).
> example: the very *first* user that appears in the terminal commands listed above is the owner of the contract we are calling. The user appearing right after the `--accountId` flag is the signer of the transaction.

---
#### Debugging 
###### Compile time errors
You must compile the smart contract before deploying it to blockchain. Compile the contract running the terminal command `./build.sh`. If compilation returns an error *unable to get packages from source* you might need to clear the cargo registry running `rm -rf /<userpathtocargoregistry>/.cargo/registry/`.


#### Using `near_sdk` Persistent Collections

> Note to NEAR Rust developers: remember to choose your Rust objects based on their associated time complexity. Consult [this](https://docs.near.org/docs/concepts/data-storage#big-o-notation-1) table ranking object types in the `near_sdk' Rust collection by Big-O Notation.
> Remember that all objects (structs, enums, etc.) which 'live' on-chain, should preferably be objects in the NEAR persistet collections, whereas objects that 'live' off-chain *must* be Rust std collections or Rust objects of some sort. near_sdk objects only exist on-chain and can't be rendered off-chain.
