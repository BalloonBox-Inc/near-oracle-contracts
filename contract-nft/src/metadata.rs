use crate::*;
use near_sdk::{Gas};
pub type TokenId = String;
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
//defines the payout type we'll be returning as a part of the royalty standards.
pub struct Payout {
    pub payout: HashMap<AccountId, U128>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct NFTContractMetadata {
    pub spec: String,              // required, essentially a version like "nft-1.0.0"
    pub name: String,              // required, ex. "Mosaics"
    pub symbol: String,            // required, ex. "MOSIAC"
    pub timestamp: u64,
    pub icon: Option<String>,      // Data URL
    pub base_uri: String, // Centralized gateway known to have reliable access to decentralized storage assets referenced by `reference` or `media` URLs
    pub reference: Option<String>, // URL to a JSON file with more info
    pub reference_hash: Option<Base64VecU8>, // Base64-encoded sha256 hash of JSON from reference field. Required if `reference` is included.
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenMetadata {
    pub title: String, // ex. "Arch Nemesis: Mail Carrier" or "Parcel #5055"
    pub description: String, // free-form description
    pub media: String, // URL to associated media, preferably to decentralized, content-addressed storage
    pub media_hash: Option<Base64VecU8>, // Base64-encoded sha256 hash of content referenced by the `media` field. Required if `media` is included.
    pub copies: Option<u64>, // number of copies of this set of metadata in existence when token was minted.
    pub issued_at: Option<u64>, // When token was issued or minted, Unix epoch in milliseconds
    pub expires_at: Option<u64>, // When token expires, Unix epoch in milliseconds
    pub starts_at: Option<u64>, // When token starts being valid, Unix epoch in milliseconds
    pub updated_at: Option<u64>, // When token was last updated, Unix epoch in milliseconds
    pub extra: Option<String>, // anything extra the NFT wants to store on-chain. Can be stringified JSON.
    pub reference: Option<String>, // URL to an off-chain JSON file with more info.
    pub reference_hash: Option<Base64VecU8>, // Base64-encoded sha256 hash of JSON from reference field. Required if `reference` is included.
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Token {
    //define token owner
    pub owner_id: AccountId,
    //list of approved account IDs that have access to transfer the token. This maps an account ID to an approval ID
    pub approved_account_ids: HashMap<AccountId, u64>,
    //the next approval ID
    pub next_approval_id: u64,
    //perfentage of royalty to be paid to an account
    pub royalty: HashMap<AccountId, u32>,
}

//The Json token is what will be returned from view calls. This object exists off-chain only. It holds all the information
//for an NFT that you want to send back as JSON whenever someone does a view call
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct JsonToken {
    //token id
    pub token_id: TokenId,
    //token owner
    pub owner_id: AccountId,
    //token metadata
    pub metadata: TokenMetadata,
    // list of approved account IDs that have access to transfer the token. This maps an account ID to an approval ID
    pub approved_account_ids: HashMap<AccountId, u64>,
    //perfentage of royalty to be paid to an account
    pub royalty: HashMap<AccountId, u32>,
}

// was the operation of minting a score as NFT successful?
// the struct below describes the operation outcome
#[derive(Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct MintOutcome {
    pub gas_used: Gas,
    pub nft_id: TokenId,
    pub owner_id: AccountId,
    pub successful_operation: bool,
}

/*
Imagine we want a funciton for quering contract metadata. Create it following this logic:
- create a trait containing your desired function
- implement that trait on the 'Contract' struct
- so that whenever we call the 'Contract' struct the function is also executed
 */
pub trait NonFungibleTokenMetadata {
    //view call for returning the contract metadata
    fn nft_metadata(&self) -> NFTContractMetadata;
}

#[near_bindgen]
impl NonFungibleTokenMetadata for Contract {
    fn nft_metadata(&self) -> NFTContractMetadata {
        self.metadata.get().unwrap()
    }
}