/* unit tests */
#[cfg(test)]
use crate::Contract;
use crate::{NFTContractMetadata, TokenMetadata};
use near_sdk::testing_env;
use near_sdk::json_types::{U128};
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

fn meta() -> TokenMetadata {
    TokenMetadata {
        title: "Test NFT".to_string(),
        description: "A minted NFT".to_string(),
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
    let mut context = get_context(
        false, 
        account("doomslug.testnet")
    );
    testing_env!(context);
    let mut contract = Contract::new_default_meta(
        account("doomslug.testnet")
    );

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
    let mut context = get_context(
        false, 
        account("doomslug.testnet")
    );
    testing_env!(context.clone());
    let mut contract = Contract::new_default_meta(
        account("doomslug.testnet")
    );

    //try ot add to the whitelist by a NOT whitelisted account id -> should panic
    context.predecessor_account_id = account("benji.testnet");
    testing_env!(context.clone());
    assert!(contract.add_to_whitelist(&account("rainbow.testnet")));
}


#[test]
fn test_init() {
    //set up the testing context
    let mut context = get_context(
        false, account("doomslug.testnet")
    );
    testing_env!(context.clone());

    let metadata = NFTContractMetadata {
        spec: "nft-contract".to_string(),
        name: "NearOracle NFT minter".to_string(),
        symbol: "Balloonbox".to_string(),
        timestamp: env::block_timestamp(),
        icon: None,
        base_uri: None,
        reference: None,
        reference_hash: None,
    };
    let mut contract = Contract::new(
        account("benjiman.testnet"), metadata
    );

    assert_eq!(account("benjiman.testnet"), contract.owner_id, 
        "ERR: Owner should be benjiman at initialization");
    assert!(!contract.metadata.is_none(),
        "ERR: contract metadata is not initialized");
    assert_eq!(U128(0), contract.nft_total_supply(),
        "ERR: NFT count should be 0 at initialization");
    assert_eq!(0, contract.nft_tokens(None, None).len(),
        "ERR: Token count should be 0 at initialization");
    assert!(contract.token_metadata_by_id.is_empty(),
        "ERR: This map should be empty at initialization");
}




#[test]
fn test_enums() {
    //set up the testing context
    let mut context = get_context(
        false, 
        account("doomslug.testnet")
    );
    testing_env!(context.clone());
    let mut contract = Contract::new_default_meta(
        account("doomslug.testnet")
    );

    let s = account("spensa.testnet");
    let r = account("rainbow.testnet");
    let s1 = account("spensa.testnet");
    let r1 = account("rainbow.testnet");
    context.attached_deposit = u128::pow(10, 23);
    testing_env!(context.clone());
    contract.nft_mint("001".to_string(), meta(), s, None);
    context.attached_deposit = u128::pow(10, 23);
    testing_env!(context.clone());
    contract.nft_mint("002".to_string(), meta(), s1, None);
    context.attached_deposit = u128::pow(10, 23);
    testing_env!(context.clone());
    contract.nft_mint("003".to_string(), meta(), r, None);


    assert_eq!(U128(3), contract.nft_total_supply());
    assert_eq!(3, contract.nft_tokens(None, None).len());
    assert_eq!(U128(2), contract.nft_supply_for_owner(&account("spensa.testnet")));
    assert_eq!(U128(1), contract.nft_supply_for_owner(&r1));
    assert_eq!(Some(r1), contract.whose_token("003".to_string()));
}