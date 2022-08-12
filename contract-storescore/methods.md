<p align="center">
  <a href="https://near.org/">
    <img alt="NearMonotoneWhite" src="https://github.com/BalloonBox-Inc/near-oracle-contracts/blob/dev/images/monotone-white-bg.png" width="1300" />
  </a>
</p>

# PUBLIC METHODS

#### About :spiral_notepad:
This document explains the purpose of the most important public and private methods (or functions) contained in the codebase of the store-score smart contract. Specifically, here we list some noteworthy methods, invoked by the frontend of the NearOracle dApp. Remember that methods can be of two types:
 - **calls: (cost gas)** these methods alter the contract state, i.e., they're state handlers
 - **views: (gasless)** these methods are view-only and are used to query the contract state without changing it

#### Help Us :handshake:
Have you spotted a mistake in our NEARoracle docs? Help us improve it by [letting us know](https://www.balloonbox.io/contact).

## Function Calls
List of state-handling functions.
```bash
    #stores a score to the Near blockchain and returns 
    #a struct indicating whether the operation was successful
    #(although this is a public method, it can only be invoked either
    #by the contract owner or by a whitelisted Near account id)
    
    #[payable]
    pub fn store_score(
        &mut self,
        score: u16,
        description: String
        ) -> ScoreOutcome { ... }


    #add an account ID to the whitelist returning `true` if the account id 
    #was not in the whitelist before, `false` otherwise.
    #This method can be called only by the smart contract owner.

    #[private]
    pub fn add_to_whitelist(
        &mut self,
        account_id: AccountId
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
> These view-only functions are stored in the Rust file [`./contract-storescore/src/lib.rs`](src/lib.rs).
> 
> N.B. The above documentation does not contain the function logic. We omitted it intentionally for readability sake, replacing it with the `{ ... }` placeholder. If you want to see the full source code, please consult the .rs files where the functions are stored.