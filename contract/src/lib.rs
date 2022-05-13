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

const USER_NUMBER: u64 = 7;

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
    pub description: u32,
}

// // a scoreboard struct containing all scores for one user
// #[derive(BorshDeserialize, BorshSerialize)]
// pub struct ScoreBoard {
//     scores_by_user: LookupSet<Score>,
// }

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

    pub fn store_score(&mut self, score: u16, timestamp: u32, description: u32) {
        let account_id = String::from(env::predecessor_account_id());
        let new_score = Score {
            score: score,
            timestamp: timestamp,
            description: description,
        };
        let mut x = LookupSet::new(b"c");
        x.insert(&new_score);
        // let y = ScoreBoard { scores_by_user: x };
        self.records.insert(&account_id, &x);
    }

    pub fn query_all_scores(&self, account_id: String) -> MyScoreHistory {
        let all_scores = self.records.get(&account_id);
        // implement logic in case the above Option<T> returns a NoneType
        let read_score = all_scores.try_to_vec().unwrap();
        MyScoreHistory { scores: read_score }
    }

    // pub fn query_all_scores(&self, account_id: String) -> Option<ScoreBoard> {
    //     return self.records.get(&account_id);
    // }

    // // read the number of user
    // pub fn get_user_number(&self) -> u64 {
    //     self.user_count
    // }

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
