use crate::*;
use near_sdk::{ext_contract, log, Gas, PromiseResult};

pub trait NonFungibleTokenCore {
    //get information about the NFT token passed in
    fn json_token(&self, token_id: TokenId) -> Option<JsonToken>;
}

#[near_bindgen]
impl NonFungibleTokenCore for Contract {

    //get the information for a specific token ID. Return an 'Option', depending on whether a token exists or not
    fn json_token(&self, token_id: TokenId) -> Option<JsonToken> {
        //if there is some token ID in the tokens_by_id collection
        if let Some(token) = self.token_by_id.get(&token_id) {
            //then get then metadata for that token
            let metadata = self.token_metadata_by_id.get(&token_id).unwrap();
            //return the JsonToken (wrapped by Some since we return an option)
            Some(JsonToken {
                token_id,
                owner_id: token.owner_id,
                metadata,
                // approved_account_ids: token.approved_account_ids,
                // royalty: token.royalty,
            })

        //if there is no token ID in the token_by_id_collection, then return None
        } else {
            None
        }
    }
}