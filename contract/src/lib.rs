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

static USER_COUNT: u64 = 0;
static SCORE_COUNT: u64 = 0;
const MAX_SIZE: u64 = 5000000000000;

// on-chain struct describing the current state of the smart contract
#[derive(BorshDeserialize, BorshSerialize)]
pub struct State {
    pub max_size: u64,
    pub size_now: u64,
    pub user_count: u64,
    pub score_count: u64,
}

// off-chain Vec<u64> returning the contract state in a human-readable format
#[derive(Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ContractState {
    state_now: Vec<u64>,
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
    pub timestamp: u32,
    pub description: Vec<u8>,
}

// singleton and main struct for this smart contract
#[near_bindgen]
#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    owner_id: AccountId,
    records: LookupMap<String, Vector<Score>>,
}

// --------------------------------------------------------------------- //
//                        Implement main objects                         //
//                                                                       //
// ----------------------------------------------------------------------//
impl Default for State {
    fn default() -> Self {
        Self {
            max_size: 1000000,
            size_now: env::storage_usage(),
            user_count: 0,
            score_count: 0,
        }
    }
}

#[near_bindgen]
impl Contract {
    // initialize the contract
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        Self {
            owner_id,
            records: LookupMap::new(b"s"),
        }
    }

    // -----------------------------------------------------//
    //              Score-related implementations           //
    // -----------------------------------------------------//
    // store a new score to blockchain
    pub fn store_score(&mut self, score: u16, timestamp: u32, description: Vec<u8>) -> String {
        let account_id = String::from(env::predecessor_account_id());
        let new_score = Score {
            score: score,
            timestamp: timestamp,
            description: description,
        };

        if env::storage_usage() < MAX_SIZE {
            let mappy = self.records.get(&account_id);
            match mappy {
                // if it's a new user --> create a brand new vector to store their score
                None => {
                    let mut x = Vector::new(b"v");
                    x.push(&new_score);
                    self.records.insert(&account_id, &x);
                    // USER_COUNT += 1;
                }
                // if it's a returning user --> append new score to existing vector
                Some(i) => {
                    if i.len() < 10 {
                        let mut y = i;
                        y.push(&new_score);
                        self.records.insert(&account_id, &y);
                    } else {
                        env::panic_str("ERR_EXCEEDED_TEN_SCORES_UPPERBOUND")
                    }
                }
            }
            // SCORE_COUNT += 1  // update the score count iff you succeeded writing it to chain
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
            MyScoreHistory {
                scores: read_scores,
            }
        } else {
            // implement logic in case the above Option<T> returns a NoneType
            env::panic_str("ERR_THIS_USER_HAS_NO_SCORE_HISTORY")
        }
    }

    // -----------------------------------------------------//
    //              State-related implementations           //
    // -----------------------------------------------------//
    // query total number of scores stored on chain
    pub fn get_max_size(&self) -> u64 {
        return MAX_SIZE;
    }

    // read the number of users and scores
    pub fn get_stats(&self) -> Vec<u64> {
        let stats: Vec<u64> = vec![USER_COUNT, SCORE_COUNT];
        stats
    }

    // // STATE
    // // update the state of the contract
    // pub fn set_state(&mut self) {
    //     // max_size remains equal to its initialization value
    //     self::size_now = env::storage_usage();
    //     self.user_count += 1;
    //     self.score_count += 1;
    // }

    // pub fn get_state(&self) -> u64 {
    //     return self.size_now;
    // }

    // pub fn read_state(&self) -> ContractState {
    //     let mut info = vec![];
    //     info.push(self.max_size);
    //     info.push(self.size_now);
    //     info.push(self.user_count);
    //     info.push(self.score_count);
    //     ContractState { state_now: info }
    // }
}
