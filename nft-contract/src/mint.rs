use crate::*;

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn nft_mint(&mut self, token_id: TokenId, metadata: TokenMetadata, receiver_id: AccountId) {
        // measure the initial storage being used on the contract
        let initial_storage_usage = env::storage_usage();

        // specify the token struct that contains the owner ID
        let token = Token {
            // set owner ID to be equal to the receiver ID
            owner_id: receiver_id,
            // set the approved account IDs to the default value (an empty map)
            approved_account_ids: Default::default(),
            // the next approval ID is set to 0
            next_approval_id: 0,
        };

        // insert the token ID and the token struct,
        // but first make sure that the token doen't exist -> do this latter part by using
        // the 'assert!' macro with a custom panic message
        assert!(
            self.token_by_id.insert(&token_id, &token).is_none(),
            "Token already exists"
        );

        // insert token id and metadata
        self.token_metadata_by_id.insert(&token_id, &metadata);

        //call an internal method to add a token to the owner
        self.internal_add_token_to_owner(&token.owner_id, &token_id);

        //calculate the required storage = used storage - initial storage
        let required_storage_in_bytes = env::storage_usage() - initial_storage_usage;

        // refund surplus storage to user OR panic if they didn't attach enough to cover for the required gas fee
        refund_deposit(required_storage_in_bytes);
    }
}