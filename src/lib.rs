// Import crates
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};

// // Declare a global variable
// const PUZZLE_NUMBER: u8 = 3;

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    crossword_solution: String, // SETUP CONTRACT STATE
}

#[near_bindgen]
impl Contract {
    // ADD CONTRACT METHODS HERE

    // create a new 'Contract' object
    #[init]
    pub fn new(solution: String) -> Self {
        Self {
            crossword_solution: solution,
        }
    }

    // // add a view-only function
    // pub fn get_puzzle_number(&self) -> u8 {
    //     PUZZLE_NUMBER
    // }

    // costless query the state of the contract
    pub fn get_solution(&self) -> String {
        self.crossword_solution.clone()
    }

    // // add 2 change-method functions: set a solution
    // pub fn set_solution(&mut self, solution: String) {
    //     self.crossword_solution = solution;
    // }

    // check whether guessed solution is correct
    pub fn guess_solution(&mut self, solution: String) -> bool {
        let hashed_input = env::sha256(solution.as_bytes());
        let hashed_input_hex = hex::encode(&hashed_input);

        if hashed_input_hex == self.crossword_solution {
            env::log_str("You guessed right");
            true
        } else {
            env::log_str("Try again");
            false
        }
    }
}

//___________________________________________________________________________________//
//                                                                                   //
//                                   Unit Tests                                      //
//                                                                                   //
//___________________________________________________________________________________//
/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-template -- --nocapture
 * Note: 'rust-template' comes from Cargo.toml's 'name' key
 */

// use the attribute below for unit tests
#[cfg(test)] // <-- this line prevents this module from being run unless I execute 'cargo test'
mod tests {
    use super::*;
    use near_sdk::test_utils::{get_logs, VMContextBuilder};
    use near_sdk::{testing_env, AccountId};

    #[test]
    fn debug_get_hash() {
        // Basic set up for a unit test
        testing_env!(VMContextBuilder::new().build());

        // Use a unit test to rapidly debug and iterate
        let debug_solution = "happy birthday mayllon from bboxteam";
        let debug_hash_bytes = env::sha256(debug_solution.as_bytes());
        let debug_hash_string = hex::encode(debug_hash_bytes);
        println!("Let's debug: {:?}", debug_hash_string)
    }

    // part of writing unit tests is setting up a mock context
    // provide a `predecessor` here, it'll modify the default context
    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    #[test]
    fn check_guess_solution() {
        // get your account id
        let arabella = AccountId::new_unchecked("arabellapark.testnet".to_string());
        // set up the testing context and the unit test environmnet
        let context = get_context(arabella);
        testing_env!(context.build());

        // set up the contract object
        let mut contract = Contract::new(
            "a01c354e04406da49ec20d9f258a71e050e1ea1f2756db451626be2ffd5ae13f".to_string(),
        );

        // call the guess_solution function with the incorrect solution
        let mut guess_result = contract.guess_solution("wrong answer here".to_string());
        println!("First attempt: {}", !guess_result);
        assert!(!guess_result, "Expected a failure from the wrong guess"); // 'assert' with a custom message
        assert_eq!(get_logs(), ["Try again"], "Expected a failure log."); // 'assert_eq' with a custom message

        // call the guess_solution function with the correct solution
        guess_result = contract.guess_solution("happy birthday mayllon from bboxteam".to_string());
        println!("Second attempt: {}", guess_result);
        assert!(guess_result, "Expected the correct answer to return true.");
        assert_eq!(
            get_logs(),
            ["You guessed right"],
            "Expected a successful log after the previous failed log."
        );
    }
}
