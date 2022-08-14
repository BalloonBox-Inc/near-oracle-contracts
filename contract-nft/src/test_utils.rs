/* unit tests */
#[cfg(test)]
use crate::Contract;
use crate::TokenMetadata;
use near_sdk::testing_env;
use near_sdk::{env, AccountId, VMContext};
use near_sdk::test_utils::{VMContextBuilder};

fn account(account_name: &str) -> AccountId {
    account_name.to_string().try_into().unwrap()
}

//set up a mock context for testing
fn get_context(is_view: bool, predecessor: AccountId) -> VMContext {
    VMContextBuilder::new()
        .current_account_id(account("spensa.testnet"))
        .signer_account_id(predecessor.clone())
        .predecessor_account_id(predecessor.clone())
        .is_view(is_view)
        .build()
}

// fn sample_token_metadata() -> TokenMetadata {
//     TokenMetadata {
//         title: "NFT #1".to_string(),
//         description: "First NFT".to_string(),
//         media: "https://near.org/wp-content/uploads/2021/09/brand-stack-300x300.png".to_string(),
//         media_hash: None,
//         copies: None,
//         issued_at: Some(env::block_timestamp()),
//         expires_at: None,
//         starts_at: None,
//         updated_at: None,
//         extra: None,
//         reference: None,
//         reference_hash: None,
//     }
// }


#[test]
fn test_whitelist() {
    let mut context = get_context(false, account("doomslug.testnet"));
    testing_env!(context);
    let mut contract = Contract::new_default_meta(account("doomslug.testnet"));

    //Verify owner identity
    assert_eq!("spensa.testnet".to_string(), String::from(env::current_account_id()));
    assert_eq!("doomslug.testnet".to_string(), String::from(env::predecessor_account_id()));
    let a = account("rainbow.testnet");

    //Adding to list by foundation
    assert!(contract.add_to_whitelist(&a));

    //Checking it's whitelisted now
    assert!(contract.whitelist.contains(&a));

    //Adding again. Should return false
    assert!(!contract.add_to_whitelist(&a));

    //Checking it's still whitelisted
    assert!(contract.whitelist.contains(&a));
}


#[test]
#[should_panic(expected = "This function can only be called by the contract owner")]
fn test_whitelist_fail() {
    let mut context = get_context(false, account("doomslug.testnet"));
    testing_env!(context.clone());
    let mut contract = Contract::new_default_meta(account("doomslug.testnet"));

    // Trying ot add to the whitelist by NOT whitelisted factory.
    context.predecessor_account_id = account("benji.testnet");
    testing_env!(context.clone());
    assert!(contract.add_to_whitelist(&account("rainbow.testnet")));
}