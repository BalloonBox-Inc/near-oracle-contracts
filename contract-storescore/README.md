<p align="center">
  <a href="https://near.org/">
    <img alt="Near" src="https://github.com/BalloonBox-Inc/near-oracle-contracts/blob/dev/images/inverted-primary-logo-bg.png" width="700" />
  </a>
</p>

## Storing credit scores :1st_place_medal: :2nd_place_medal: :3rd_place_medal:
What does this smart contract do?
 - stores to blockchain (in a map) the credit scores of a user (max 1 score/month)
 - query a user's credit score history, e.g., to monitor improvements
 - query how many credit scores a user owns
 - reads contract state


# PUBLIC METHODS
---

#### About :spiral_notepad:
The NFT-minter smart contract contains numerous public methods or functions, callable from outside of the contract. This documentations lists a few noteworthy methods, namely some of the ones invoked by the dApp as the frontend interacts with the smart contract. Methods can be of two types:
 - **calls: (cost gas)** these methods alter the contract state, i.e., they're state handlers
 - **views: (gasless)** these methods are view-only and are used to query the contract state without changing it

> **Help Us:** :handshake: Have you spotted a mistake in our NEARoracle docs? Help us improve it by [letting us know](https://www.balloonbox.io/contact).

## Function Calls
List of state-handling functions.
```bash
    #stores a score to the Near blockchain and returns 
    #a struct indicating whether the operation was successful
    #(although this is a public method, it can only be invoked either
    #by the contract owner or by a whitelisted Near account id)
    pub fn store_score(
        &mut self,
        score: u16,
        description: String
        ) -> ScoreOutcome { ... }


    #add an account ID to the whitelist returning `true` if the account id 
    #was not in the whitelist before, `false` otherwise.
    #This method can be called only by the smart contract owner.
    pub fn add_to_whitelist(
        &mut self,
        account_id: &AccountId
        ) -> bool { ... }
```
> Find the complete code of the *store_scoret()* function in the file [`./contract-storescore/src/lib.rs`](src/lib.rs), whereas *add_to_whitelist()* is found in [`./contract-storescore/src/whitelist.rs`](src/whitelist.rs).

## View Calls
List of view-only functions.
```bash
#query the entire score history of a Near account id
#it returns a struct containg a vec<Score> where Score is itself a struct
pub fn query_score_history(
    &self,
    account_id: String
    ) -> MyScoreHistory { ... }

#return the contract state at a point in time
pub fn read_state(&self) -> ContractState { ... }

#check whether a user has a score record
pub fn user_exist(
    &self,
    account_id: String
    ) -> bool { ... }

#return the total number of scores of the account id you pass in
pub fn maxout_check(
    &self,
    account_id: String
    ) -> u64 { ... }
```
> These view-only functions are stored in [`./contract-storescore/src/lib.rs`](src/lib.rs).

> N.B. The above documentation does not contain the function logic. We omitted it intentionally for readability sake, replacing it with the `{ ... }` placeholder. If you want to see the full source code, please consult the .rs files where the functions are stored.


## Pricing
How much gas does it cost to call a smart contract method? Our estimates follow. Remember that the gas price on the Near blockchain fluctuates over time; see docs on [Near Gas](https://docs.near.org/concepts/basics/transactions/gas). All view functions are free of charge.

|Method|Call Type|Gas|
|:-----:|:-----:|:-----:|:-----:|
|`store_score`|call|0.65m Ⓝ|
|`add_to_whitelist`|call|0.550m Ⓝ|
|`remove_from_whitelist`|call|0.550m Ⓝ|





