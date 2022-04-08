// Import crates
use near_sdk::collections::{LookupMap, UnorderedSet};
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    log,
    serde::{Deserialize, Serialize},
    AccountId, PanicOnDefault, Promise,
};
use near_sdk::{env, near_bindgen};

// 5 Ⓝ in yoctoNEAR
const PRIZE_AMOUNT: u128 = 5_000_000_000_000_000_000_000_000;

//___________________________________________________________________________________//
//                                                                                   //
//                                  Structs & Enums                                  //
//                                                                                   //
//___________________________________________________________________________________//

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum PuzzleStatus {
    Unsolved,
    Solved { memo: String },
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum AnswerDirection {
    Across,
    Down,
}

// Coordinates of where the crossword begins
#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct CoordinatePair {
    x: u8,
    y: u8,
}

// Info on puzzle answer
#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Answer {
    num: u8,
    start: CoordinatePair,
    direction: AnswerDirection,
    length: u8,
    clue: String,
}

// Puzzle status
#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct Puzzle {
    status: PuzzleStatus, // ⟵ An enum we'll get to soon
    answer: Vec<Answer>,  // ⟵ Another struct we've defined
}

// Same Puzzle as above w/ human-readable (not in bytes) hash of the crossword solution
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct JsonPuzzle {
    solution_hash: String,
    status: PuzzleStatus,
    answer: Vec<Answer>,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
// 'Crossword' is our primary struct or singleton.
// Remember the singleton is the ONLY struct that ALWAYS gets the #[near_bindgen] macro placed on it
pub struct Crossword {
    // crossword_solution: String, // SETUP CONTRACT STATE
    // add an owner_id because it's common in smart contract development to implement a rudimentary permission system which can restrict access to certain functions
    owner_id: AccountId,
    puzzles: LookupMap<String, Puzzle>, // ⟵ 'Puzzle' is a struct we're defining
    unsolved_puzzles: UnorderedSet<String>,
}

// Check user balance
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct StorageBalance {
    pub total: u128,
    pub available: u128,
}

//___________________________________________________________________________________//
//                                                                                   //
//                              Implementations                                      //
//                                                                                   //
//___________________________________________________________________________________//
#[near_bindgen]
impl Crossword {
    // ADD CONTRACT METHODS HERE

    // create a new 'Corossword' object
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        Self {
            owner_id,
            puzzles: LookupMap::new(b"c"),
            unsolved_puzzles: UnorderedSet::new(b"u"),
        }
    }

    // create a new puzzle method to insert multiple crosswords
    pub fn new_puzzle(&mut self, solution_hash: String, answers: Vec<Answer>) {
        // first thing that happens in the new_puzzle method is a check
        // we check that the predecessor (whoever called this method last) is indeed the contract owner
        assert_eq!(
            env::predecessor_account_id(),
            self.owner_id,
            "Only the owner may call this method"
        ); // if someone else other than the owner is trying to call new_puzzle, the smart contract will panic,
           // going no further. Hence, notice that assert_eq! breaks contract executions when it panics.

        let existing = self.puzzles.insert(
            &solution_hash,
            &Puzzle {
                status: PuzzleStatus::Unsolved,
                answer: answers,
            },
        );

        // perform another check: add a new puzzle only if it's not a duplicate of a pre-existing one
        assert!(existing.is_none(), "Puzzle with that key already exists");
        self.unsolved_puzzles.insert(&solution_hash);
    }

    // submit solution to the net
    pub fn submit_solution(&mut self, solution: String, memo: String) {
        let hashed_input = env::sha256(solution.as_bytes());
        let hashed_input_hex = hex::encode(&hashed_input);

        // Check to see if the hashed answer is among the puzzles
        let mut puzzle = self
            .puzzles
            .get(&hashed_input_hex)
            .expect("ERR_NOT_CORRECT_ANSWER");

        // Check if the puzzle is already solved. If it's unsolved, set the status to solved,
        //   then proceed to update the puzzle and pay the winner.
        puzzle.status = match puzzle.status {
            PuzzleStatus::Unsolved => PuzzleStatus::Solved { memo: memo.clone() },
            _ => {
                env::panic_str("ERRO_PUZZLE_SOLVED");
            }
        };

        // Reinsert the puzzle back in after we modified the status
        self.puzzles.insert(&hashed_input_hex, &puzzle);
        // Remove from the list of unsolved ones
        self.unsolved_puzzles.remove(&hashed_input_hex);

        log!(
            "Puzzle with solution hash {} solved, with memo {}",
            hashed_input_hex,
            memo
        );

        // Transfer the prize money to the winner
        Promise::new(env::predecessor_account_id()).transfer(PRIZE_AMOUNT);
    }

    // // costless query the state of the contract
    // // CHALLENGE: try and query for free ALL solutions from the contract
    // // performs a check to see whether the querier is the contract owner
    // pub fn get_solution(&self) -> String {
    //     self.crossword_solution.clone()
    // }
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
