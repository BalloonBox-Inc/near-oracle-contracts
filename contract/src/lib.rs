// Import crates
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};

// --------------------------------------------------------------------- //
//                          Define main objects                          //
//                                                                       //
// ----------------------------------------------------------------------//

const USER_NUMBER: u64 = 2;

// struct on the current state of the smart contract
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct State {
    pub max_size: u64,
    pub size_now: u64,
    pub user_count: u64,
    pub score_count: u64,
}

impl Default for State {
    fn default() -> Self {
        Self {
            max_size: 3000,
            size_now: env::storage_usage(),
            user_count: USER_NUMBER,
            score_count: 5,
        }
    }
}

// --------------------------------------------------------------------- //
//                        Implement main objects                         //
//                                                                       //
// ----------------------------------------------------------------------//
#[near_bindgen]
impl State {
    // read the number of user
    pub fn get_user_number(&self) -> u64 {
        self.user_count
    }

    // update the state of the contract
    pub fn set_state(&mut self, max_size: u64, score_count: u64) {
        self.max_size = max_size;
        self.size_now = env::storage_usage();
        self.user_count = USER_NUMBER + 100;
        self.score_count = score_count;
    }

    pub fn get_state(&self) -> String {
        return String::from("active state");
    }
}