use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedSet};
use near_sdk::{env, near_bindgen};

// struct on the current state of the smart contract
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct State {
    pub max_size: u16,
    pub user_count: u64,
    pub score_count: u64,
}

// user's score, timestamp, and score descriptor as a struct
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct User {
    pub score: u16,
    pub timestamp: u32,
    pub description: Vec<u8>,
}

// main struct of the contract storing the all-time scoreboard
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    pub score_board: LookupMap<String, UnorderedSet<User>>,
}

// // TBD: AccoundId, which will be a string

// #[near_bindgen]
// #[derive(BorshDeserialize, BorshSerialize)]
// pub struct StatusMessage {
//     records: LookupMap<String, String>,
// }

// impl Default for StatusMessage {
//     fn default() -> Self {
//         Self {
//             records: LookupMap::new(b"r".to_vec()),
//         }
//     }
// }

// #[near_bindgen]
// impl StatusMessage {
//     pub fn set_status(&mut self, message: String) {
//         let account_id = String::from(env::signer_account_id());
//         self.records.insert(&account_id, &message);
//     }

//     pub fn get_status(&self, account_id: String) -> Option<String> {
//         return self.records.get(&account_id);
//     }
// }
