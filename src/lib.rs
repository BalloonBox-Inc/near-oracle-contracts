// Import crates
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};

// Declare a global variable
const PUZZLE_NUMBER: u8 = 3

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    crossword_solution: String,
    // SETUP CONTRACT STATE
}

#[near_bindgen]
impl Contract {
    // ADD CONTRACT METHODS HERE

    // add a view-only function
    pub fn get_puzzle_number(&self) -> u8 {
        PUZZLE_NUMBER
    }

    // add 2 change-method functions
    pub fn set_solution(&mut self, solution: String) {
        self.crossword_solution = solution;
    }

    pub fn guess_solution(&mut self, solution: String) {
        if self.crossword_solution == solution {
            env::log_str("You guessed right")
        } else {
            env::log_str("Try again")
        }
    }
}

/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-template -- --nocapture
 * Note: 'rust-template' comes from Cargo.toml's 'name' key
 */

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{get_logs, VMContextBuilder};
    use near_sdk::{testing_env, AccountId};

    // part of writing unit tests is setting up a mock context
    // provide a `predecessor` here, it'll modify the default context
    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    // TESTS HERE
}
