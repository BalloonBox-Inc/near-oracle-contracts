use near_sdk::{env, AccountId, near_bindgen};
use crate::*;


#[near_bindgen]
impl Contract {
    //Internal method to verify the predecessor was the smart contract owner
    fn assert_called_by_owner(&self) {
        assert_eq!(
            &env::predecessor_account_id(),
            &self.owner_id,
            "This function can only be called by the contract owner"
        );
    }

    //Adds the given account ID to the whitelist.
    //Returns `true` if the account id was not in the whitelist before, `false` otherwise.
    //This method can be called only by the smart contract owner.
    #[private]
    pub fn add_to_whitelist(&mut self, account_id: &AccountId) -> bool {

        //ensure the function was called by the smart contract owner, else panic
        self.assert_called_by_owner();

        self.whitelist.insert(&account_id)
    }
}