/* unit tests */
#[cfg(test)]
use crate::Contract;
use crate::TokenMetadata;
use crate::whitelist::*;
use near_sdk::json_types::{U128, U64};
use near_sdk::test_utils::{accounts, VMContextBuilder};
use near_sdk::{env, testing_env, AccountId};

use std::collections::HashMap;


fn account(account_name: String) -> AccountId {
    account_name.to_string().try_into().unwrap()
}

//set up a mock context for testing
fn get_context(is_view: bool, predecessor: AccountId) -> VMContext {
    VMContextBuilder::new()
        .current_account_id(account("doomslug.testnet"))
        .signer_accoount_id(accout(predecessor))
        .predecessor_account_id(account(predecessor))
        .is_view(is_view)
        .build()
}

fn sample_token_metadata() -> TokenMetadata {
    TokenMetadata {
        title: "NFT #1".to_string(),
        description: "First NFT".to_string(),
        media: "https://near.org/wp-content/uploads/2021/09/brand-stack-300x300.png".to_string(),
        media_hash: None,
        copies: None,
        issued_at: Some(env::block_timestamp()),
        expires_at: None,
        starts_at: None,
        updated_at: None,
        extra: None,
        reference: None,
        reference_hash: None,
    }
}

#[test]
fn test_whitelist() {
    let mut context = get_context(false, "spensa.testnet");
    testing_env!(context);
    let contract = Contract::new_default_meatdata(accounts(1));

    //Verify owner identity
    assert_eq!("doomslug.testnet".to_string(), String::from(env::current_account_id()));
    assert_eq!("spensa.testnet".to_string(), String::from(env::predecessor_account_id()));
    
    //Adding to list by foundation
    assert!(contract.add_to_whitelist(account("rainbow.testnet")));

    //Checking it's whitelisted now
    assert!(contract.whitelist.contains(&AccountId::from("rainbow.testnet")));

    //Adding again. Should return false
    assert!(!contract.add_to_whitelist(account("rainbow.testnet")));

    //Checking it's still whitelisted
    assert!(contract.whitelist.contains(&AccountId::from("rainbow.testnet")));
}