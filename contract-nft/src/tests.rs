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
fn get_context(name: AccountId) -> VMContext {
    VMContextBuilder::new()
        .current_account_id(account("spensa.testnet"))
        .signer_account_id(name.clone())
        .predecessor_account_id(name.clone())
        .build()
}

//construct sample TokenMetadata struct
fn meta(mymedia: &str) -> TokenMetadata {
    TokenMetadata {
        title: "Test NFT".to_string(),
        description: "A minted NFT".to_string(),
        media: [mymedia, ".png"].join("").to_string(),
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

//unit tests start here
#[test]
fn test_whitelist() {
    //set up the testing context
    let mut context = get_context( 
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
    //set up the testing context
    let mut context = get_context( 
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
        account("doomslug.testnet")
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

    //check initialization matches expectations
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
        account("doomslug.testnet")
    );
    testing_env!(context.clone());
    let mut contract = Contract::new_default_meta(
        account("doomslug.testnet")
    );

    //declare account ids
    let s = account("spensa.testnet");
    let r = account("rainbow.testnet");
    let s1 = account("spensa.testnet");
    let r1 = account("rainbow.testnet");

    //mint an NFT attaching a deposit
    context.attached_deposit = u128::pow(10, 23);
    testing_env!(context.clone());
    contract.nft_mint(
        "001".to_string(), 
        meta("nft#1"),
        s, 
        None
    );

    //mint a second NFT
    context.attached_deposit = u128::pow(10, 23);
    // testing_env!(context.clone());
    contract.nft_mint(
        "002".to_string(),
        meta("nft#2"), 
        s1, 
        None
    );

    //mint a third NFT
    context.attached_deposit = u128::pow(10, 23);
    // testing_env!(context.clone());
    contract.nft_mint(
        "003".to_string(),
        meta("nft#3"),
        r,
        None
    );

    //enumeration methods should return the correct count of minted NFTs
    assert_eq!(U128(3), contract.nft_total_supply());
    assert_eq!(3, contract.nft_tokens(None, None).len());
    assert_eq!(U128(2), contract.nft_supply_for_owner(&account("spensa.testnet")));
    assert_eq!(U128(1), contract.nft_supply_for_owner(&r1));
    assert_eq!(Some(r1), contract.whose_token("003".to_string()));
}


#[test]
pub fn test_mint() {
    //set up the testing context
    let mut context = get_context(
        account("doomslug.testnet")
    );
    testing_env!(context.clone());
    let mut contract = Contract::new_default_meta(
        account("doomslug.testnet")
    );

    //mint first NFT attaching a deposit
    let b = account("bob.testnet");
    context.attached_deposit = u128::pow(10, 23);
    testing_env!(context.clone());
    contract.nft_mint(
        "001".to_string(),
        meta("nft#1"), 
        b,
        None
    );

    //ensure that token parameters (owner, media, title, timestamp) match expectations
    let token1 = contract.token_by_id.get(&"001".to_string());
    if let Some(i) = token1 {
        assert_eq!(account("bob.testnet"), i.owner_id,
            "ERR: token owner mismatch");
    };
    let meta1 = contract.token_metadata_by_id.get(&"001".to_string());
    if let Some(i) = meta1 {
        assert_eq!("nft#1.png".to_string(), i.media,
            "ERR: token media mismatch");    
        assert_eq!("Test NFT".to_string(), i.title,
            "ERR: token title mismatch");
        assert!(!i.issued_at.is_none());
    };

    //mint second NFT attaching a deposit
    let spensa = account("spensa.testnet");
    contract.add_to_whitelist(&spensa);
    context.signer_account_id = spensa.clone();
    testing_env!(context.clone());

    let token2 = contract.nft_mint(
        "002".to_string(),
        meta("nft#2"), 
        spensa.clone(),
        None
    );
    assert!(!contract.whitelist.contains(&spensa));
    assert!(token2.successful_operation);
    assert_eq!("002".to_string(), token2.nft_id);
}


#[test]
#[should_panic(expected = "Duplicate error: you can't mint the same NFT twice")]
pub fn test_mint_duplicate() {
    //set up the testing context
    let mut context = get_context(
        account("doomslug.testnet")
    );
    testing_env!(context.clone());
    let mut contract = Contract::new_default_meta(
        account("doomslug.testnet")
    );
    let s = account("spensa.testnet");


    //try mint the same NFT twice
    context.attached_deposit = u128::pow(10, 23);
    testing_env!(context.clone());
    contract.nft_mint(
        "001".to_string(),
        meta("duplicate-nft"), 
        s.clone(),
        None
    );
    context.attached_deposit = u128::pow(10, 23);
    testing_env!(context.clone());
    contract.nft_mint(
        "002".to_string(),
        meta("duplicate-nft"), 
        s.clone(),
        None
    );
}


#[test]
#[should_panic(expected = "Only whitelisted accounts can call this function")]
pub fn test_mint_permissionless() {
    let mut context = get_context(
        account("doomslug.testnet")
    );
    testing_env!(context.clone());
    let mut contract = Contract::new_default_meta(
        account("doomslug.testnet")
    );
    let s = account("spensa.testnet");

    //mint an NFT from a NOT whitelisted account
    context.attached_deposit = u128::pow(10, 23);
    context.signer_account_id = s.clone();
    testing_env!(context.clone());
    contract.nft_mint(
        "001".to_string(),
        meta("nft#1"), 
        s.clone(),
        None
    );
}