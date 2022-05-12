// Import crates
// use near_sdk::collections::{LookupSet, Vector};
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    serde::Serialize,
};
use near_sdk::{env, near_bindgen};

// --------------------------------------------------------------------- //
//                          Define main objects                          //
//                                                                       //
// ----------------------------------------------------------------------//

const USER_NUMBER: u64 = 7;

// struct on the current state of the smart contract
#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct State {
    pub max_size: u64,
    pub size_now: u64,
    pub user_count: u64,
    pub score_count: u64,
}

#[derive(Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ContractState {
    state_now: Vec<u64>,
}

// --------------------------------------------------------------------- //
//                        Implement main objects                         //
//                                                                       //
// ----------------------------------------------------------------------//
#[near_bindgen]
impl State {
    // initialize the contract
    #[init]
    pub fn new() -> Self {
        Self {
            max_size: 1000000,
            size_now: env::storage_usage(),
            user_count: 0,
            score_count: 0,
        }
    }

    // read the number of user
    pub fn get_user_number(&self) -> u64 {
        self.user_count
    }

    // update the state of the contract
    pub fn set_state(&mut self) {
        // max_size remains equal to its initialization value
        self.size_now = env::storage_usage();
        self.user_count += 1;
        self.score_count += 1;
    }

    pub fn get_state(&self) -> u64 {
        return self.size_now;
    }

    pub fn read_state(&self) -> ContractState {
        let mut info = vec![];
        info.push(self.max_size);
        info.push(self.size_now);
        info.push(self.user_count);
        info.push(self.score_count);
        ContractState { state_now: info }
    }
}
