// Import crates
use near_sdk::collections::{LookupMap, LookupSet, Vector};
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
const MAX_SIZE: u64 = 5000000;

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
    scores: Vec<u8>,
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
    records: LookupMap<String, LookupSet<Score>>,
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

    // query total number of scores stored on chain
    pub fn get_max_size(&self) -> u64 {
        return MAX_SIZE;
    }

    // store a new score to blockchain
    pub fn store_score(&mut self, score: u16, timestamp: u32, description: Vec<u8>) {
        let account_id = String::from(env::predecessor_account_id());
        let new_score = Score {
            score: score,
            timestamp: timestamp,
            description: description,
        };

        if MAX_SIZE < env::storage_usage() {
            let mappy = self.records.get(&account_id);
            match mappy {
                None => {
                    let mut y = LookupSet::new(b"c");
                    y.insert(&new_score);
                    self.records.insert(&account_id, &y);
                    // USER_COUNT += 1;
                }
                _ => {
                    // pass as temporary solution
                    // let mut x = Some.insert(&new_score);
                    // self.records.insert(&account_id, &x);
                }
            }
            // SCORE_COUNT += 1  // update the score count iff you succeeded writing it to chain
        } else {
            env::panic_str("ERR_MAXED_OUT_MEMORY")
        }
    }

    // query all score history for a specified user
    pub fn query_all_scores(&self, account_id: String) -> MyScoreHistory {
        let all_scores = self.records.get(&account_id);
        // implement logic in case the above Option<T> returns a NoneType
        let read_score = all_scores.try_to_vec().unwrap();
        MyScoreHistory { scores: read_score }
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
