use crate::*;

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn nft_mint(
        &mut self,
        token_id: TokenId,
        metadata: TokenMetadata,
        receiver_id: AccountId,
        //we add an optional parameter for perpetual royalties
        perpetual_royalties: Option<HashMap<AccountId, u32>>,
) {
        //measure the initial storage being used on the contract
        let initial_storage_usage = env::storage_usage();

        //create a royalty map to store in the token
        let mut royalty = HashMap::new();

        //if perpetual royalties were passed into the function:
        if let Some(perpetual_royalties) = perpetual_royalties {
            //make sure that the length of the perpetual royalties is below 5
            //since we won't have enough GAS to pay out that many people
            assert!(perpetual_royalties.len() < 5, "Cannot add more than 4 perpetual royalty amounts");

            //iterate through the perpetual royalties and insert the account and amount in the royalty map
            for (account, amount) in perpetual_royalties {
                royalty.insert(account, amount);
            }
        }

        //specify the token struct that contains the owner ID
        let token = Token {
            //set owner ID to be equal to the receiver ID
            owner_id: receiver_id,
            //set the approved account IDs to the default value (an empty map)
            approved_account_ids: Default::default(),
            //the next approval ID is set to 0
            next_approval_id: 0,
            //the map of perpetual royalties for the token (The owner will get 100% - total perpetual royalties)
            royalty,
        };

        //insert the token ID and the token struct,
        //but first make sure that the token doen't exist -> do this latter part by using
        //the 'assert!' macro with a custom panic message
        assert!(
            self.token_by_id.insert(&token_id, &token).is_none(),
            "Token already exists"
        );

        //insert token id and metadata
        self.token_metadata_by_id.insert(&token_id, &metadata);

        //call an internal method to add a token to the owner
        self.internal_add_token_to_owner(&token.owner_id, &token_id);

        //construct the mint log as per the events standard
        let nft_mint_log: EventLog = EventLog {
            //standard name ("nep171")
            standard: NFT_STANDARD_NAME.to_string(),
            //version of the standard ("nft-1.0.0")
            version: NFT_METADATA_SPEC.to_string(),
            //the data related with the event stored in a vector
            event: EventLogVariant::NftMint(vec![NftMintLog {
                //token owner
                owner_id: token.owner_id.to_string(),
                //vector of token IDs that were minted
                token_ids: vec![token_id.to_string()],
                //an optional memo to include
                memo: None,
            }]),
        };

        //log the serialized json
        env::log_str(&nft_mint_log.to_string());

        //calculate the required storage = used storage - initial storage
        let required_storage_in_bytes = env::storage_usage() - initial_storage_usage;

        //refund surplus storage to user OR panic if they didn't attach enough to cover for the required gas fee
        refund_deposit(required_storage_in_bytes);
    }
}