// Import crates
use near_sdk::collections::{LookupMap, Vector};
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    serde::{Deserialize, Serialize},
    AccountId, PanicOnDefault,
};
use near_sdk::{env, near_bindgen};

// --------------------------------------------------------------------- //
//                          Define main objects                          //
//                                                                       //
// ----------------------------------------------------------------------//

// static USER_COUNT: u64 = 0;
// static SCORE_COUNT: u64 = 0;
// const MAX_SIZE: u64 = 5000000000000;

// on-chain struct describing the current state of the smart contract
#[derive(BorshDeserialize, BorshSerialize)]
pub struct State {
    pub max_size: u64,
    pub user_count: u64,
    pub score_count: u64,
}

// off-chain struct returning the contract state in a human-readable format
#[derive(Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ContractState {
    owner: String,
    timestamp: u64,
    max_size: u64,
    size_now: u64,
    user_count: u64,
    score_count: u64,
}

#[derive(Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct MyScoreHistory {
    scores: Vec<Score>,
}

// user's score, timestamp, and score descriptor as a struct
#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Score {
    pub score: u16,
    pub timestamp: u64,
    pub description: Vec<u8>,
}

// singleton and main struct for this smart contract
#[near_bindgen]
#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    owner_id: AccountId,
    records: LookupMap<String, Vector<Score>>,
    contract_state: State,
}

// --------------------------------------------------------------------- //
//                        Implement main objects                         //
//                                                                       //
// ----------------------------------------------------------------------//

#[near_bindgen]
impl Contract {
    // initialize the contract
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        Self {
            owner_id,
            records: LookupMap::new(b"s"),
            contract_state: State {
                max_size: 5000000000,
                user_count: 0u64,
                score_count: 0u64,
            },
        }
    }

    // -----------------------------------------------------//
    //              Score-related implementations           //
    // -----------------------------------------------------//
    // store a new score to blockchain
    pub fn store_score(&mut self, score: u16, description: String) -> String {
        let account_id = String::from(env::predecessor_account_id());
        let new_score = Score {
            score: score,
            timestamp: env::block_timestamp(),
            description: env::sha256(description.as_bytes()),
        };

        // store score iff contract hasn't maxed out its memory upper bound
        if env::storage_usage() < self.contract_state.max_size {
            let mappy = self.records.get(&account_id);

            match mappy {
                // if it's a new user --> create a brand new vector to store their score
                None => {
                    let mut x = Vector::new(b"v");
                    x.push(&new_score);
                    // update the score count iff you succeeded writing it to chain`
                    self.records.insert(&account_id, &x);
                    self.contract_state.user_count += 1;
                    self.contract_state.score_count += 1;
                }

                // if it's a returning user --> append new score to existing vector
                Some(i) => {
                    let indx = i.len() - 1;
                    if let Some(j) = i.get(indx) {
                        let timelapsed = new_score.timestamp - j.timestamp;
                        // if statement w/ 2 conditions: iff there's less than 10 scores, iff last score is 30+ days old
                        if i.len() < 10 {
                            // && timelapsed > 30 * u64::pow(10, 9) { // 30 seconds
                            // && timelapsed > 2592 * u64::pow(10, 12) {  // 30 days
                            let mut y = i;
                            y.push(&new_score);
                            // update the score count iff you succeeded writing it to chain
                            self.records.insert(&account_id, &y);
                            self.contract_state.score_count += 1;
                        } else {
                            env::panic_str(
                                "ERR_EXCEEDED_TEN_SCORES_UPPERBOUND_OR_LATEST_SCORE_IS_TOO_RECENT",
                            )
                        }
                    }
                }
            }
        } else {
            env::panic_str("ERR_MAXED_OUT_MEMORY")
        }
        // return the account name. This name is the dictionary key to access user's scores
        // iff you successfully stored the score to blockchain, then return the key to access such scores
        return account_id;
    }

    // query latest score for a specified user
    pub fn query_latest_score(&self, account_id: String) -> Score {
        if let Some(i) = self.records.get(&account_id) {
            let indx = i.len() - 1;
            match i.get(indx) {
                None => env::panic_str("ERR_THIS_USER_HAS_AN_EMPTY_SCORE_RECORD"),
                Some(j) => return j,
            }
        } else {
            // panic when the account_id has no scores associated to it
            env::panic_str("ERR_NO_RECORD_FOR_THIS_USER")
        }
    }

    // query all score history for a specified user
    pub fn query_all_scores(&self, account_id: String) -> MyScoreHistory {
        if let Some(i) = self.records.get(&account_id) {
            let read_scores = i.to_vec();
            return MyScoreHistory {
                scores: read_scores,
            };
        } else {
            // implement logic in case the above Option<T> returns a NoneType
            env::panic_str("ERR_THIS_USER_HAS_NO_SCORE_HISTORY")
        }
    }

    // -----------------------------------------------------//
    //              State-related implementations           //
    // -----------------------------------------------------//

    pub fn read_state(&self) -> ContractState {
        ContractState {
            owner: String::from(env::current_account_id()),
            timestamp: env::block_timestamp(),
            max_size: self.contract_state.max_size,
            size_now: env::storage_usage(),
            user_count: self.contract_state.user_count,
            score_count: self.contract_state.score_count,
        }
    }

    // query total number of scores stored on chain - for testing only
    pub fn get_score_count(&self) -> u64 {
        return self.contract_state.score_count;
    }

    // query contract max size
    pub fn get_max_size(&self) -> u64 {
        return self.contract_state.max_size;
    }
}

/*
 * the rest of this file sets up unit tests
 * execute them running the command:
 * cargo test --package near_oracle -- --nocapture
 * Note: 'near_oracle' comes from Cargo.toml's 'name' key
 */

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::{testing_env, AccountId, VMContext};
    use std::convert::TryInto;

    fn spensa() -> AccountId {
        "spensa.testnet".to_string().try_into().unwrap()
    }

    fn doomslug() -> AccountId {
        "doomslug.testnet".to_string().try_into().unwrap()
    }

    fn yoda() -> AccountId {
        "yoda.testnet".to_string().try_into().unwrap()
    }

    // part of writing unit tests is setting up a mock context
    // provide a `predecessor` here, it'll modify the default context
    fn get_context(is_view: bool, predecessor: AccountId) -> VMContext {
        VMContextBuilder::new()
            .signer_account_id("yoda.testnet".to_string().try_into().unwrap())
            .predecessor_account_id(predecessor)
            // .block_timestamp(0u64)
            .storage_usage(0u64)
            .is_view(is_view)
            .build()
    }

    #[test]
    fn stats() {
        let context = get_context(false, doomslug());
        testing_env!(context);
        let contract = Contract::new(doomslug());

        // ensure that 'Contract' parameters are empty or null at initialization
        assert_eq!(
            0, contract.contract_state.user_count,
            "ERR: User count should be 0 at initialization"
        );
        assert_eq!(
            0, contract.contract_state.score_count,
            "ERR: Score count should be 0 at initialization"
        );
        assert_eq!(
            contract.owner_id,
            doomslug(),
            "ERR: owner ids should coincide"
        );
    }

    #[test]
    fn storing_score() {
        let context = get_context(false, yoda());
        testing_env!(context);
        let mut contract = Contract::new(yoda());

        // check initialization values are correct
        assert_eq!(0, contract.contract_state.user_count);
        assert_eq!(0, contract.contract_state.score_count);
        assert_eq!(5000000000, contract.get_max_size());
        assert_eq!(
            yoda().to_string(),
            String::from(env::predecessor_account_id())
        );

        // ensure scores are actually stored on chain

        // store first score
        let msg1 = "Congrats, your score is 570 points".to_string();
        let acc1 = contract.store_score(570, msg1);
        assert_eq!(yoda().to_string(), acc1);
        assert_eq!(1, contract.get_score_count());
        let state1 = contract.read_state();
        assert_eq!(1, state1.user_count, "ERR: should be 1 user");
        assert_eq!(1, state1.score_count, "ERR: should be 1 score");

        // store second score
        let msg2 = "Congrats, your score is 600 points".to_string();
        contract.store_score(600, msg2);
        assert_eq!(
            1, contract.contract_state.user_count,
            "ERR: should still be 1 user"
        );
        assert_eq!(
            2, contract.contract_state.score_count,
            "ERR: should be 2 scores now"
        );
    }

    #[test]
    fn querying_scores() {
        let context = get_context(false, spensa());
        testing_env!(context);
        let mut contract = Contract::new(spensa());
        contract.store_score(312, "Score of 312".to_string());
        contract.store_score(345, "Score of 345".to_string());
        contract.store_score(378, "Score of 378".to_string());

        // ensure message got sha256 encrypted
        let score_history = contract.query_all_scores("spensa.testnet".to_string());
        let last_score = contract.query_latest_score("spensa.testnet".to_string());
        assert_eq!(378, last_score.score);
        let msg3 = "Score of 378";
        let msg3_sha = env::sha256(msg3.as_bytes());
        assert_eq!(
            msg3_sha, last_score.description,
            "ERR: incorrect sha256 encryption of score descriptions"
        );
    }
}
