use crate::nft_core::NonFungibleTokenCore;
use crate::*;

#[near_bindgen]
impl Contract {

    //Who is the owner of this smart contract? Query it
    pub fn contract_owner(&self) -> AccountId {
        let owner = self.owner_id.clone();
        return owner;
    }

    //Who is the owner of a token? Query it
    pub fn whose_token(&self, token_id: TokenId) -> Option<AccountId> {
        let token = self.token_by_id.get(&token_id);
        if let Some(token) = token {
            Some(token.owner_id)
        } else {
            None
        }
    }

    //Query for the total supply of NFTs on the contract
    pub fn nft_total_supply(&self) -> U128 {
        // return the length of the token_metadata_by_id data structure
        U128(self.token_metadata_by_id.len() as u128)
    }

    //Query for nft tokens on the contract (regardless of the owner) using pagination
    pub fn nft_tokens(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<JsonToken> {
        //where to start pagination - if we have a from_index, we'll use that - otherwise start from 0 index
        let start = u128::from(from_index.unwrap_or(U128(0)));

        //iterate through each token using an iterator
        self.token_metadata_by_id.keys()
        //skip to the index we specified in the start variable
        .skip(start as usize)
        //take the first "limit" elements in the vector. If we didn't specify a limit, use 50
        .take(limit.unwrap_or(50) as usize)
        //we'll map the token IDs which are strings into Json Tokens
        .map(|token_id| self.json_token(token_id.clone()).unwrap())
        //since we turned the keys into an iterator, we need to turn it back into a vector to return
        .collect()
    }

    //get the total supply of NFTs for a given owner
    pub fn nft_supply_for_owner(&self, account_id: &AccountId) -> U128 {
        //get the set of tokens for the passed in owner
        let tokens_for_owner_set = self.tokens_per_owner.get(&account_id);
        //if there is some set of tokens, we'll return the length as a U128
        if let Some(tokens_for_owner_set) = tokens_for_owner_set {
            U128(tokens_for_owner_set.len() as u128)
        } else {
            //if there isn't a set of tokens for the passed in account ID, we'll return 0
            U128(0)
        }
    }

    /*
    Query for all the tokens for an owner. More specifically, 
    query for a paginated list of NFTs owned by a given account ID.
    This is the function that displays your NFTs among the collectibles in your NEAR wallet.
     */
    pub fn nft_tokens_for_owner(
        &self,
        account_id: &AccountId,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<JsonToken> {
        //get the set of tokens for a given owner
        let tokens_for_owner_set = self.tokens_per_owner.get(&account_id);

        //if there is some set of tokens, we'll set the tokens variable equal to that set
        let tokens = if let Some(tokens_for_owner_set) = tokens_for_owner_set {
            tokens_for_owner_set
        } else {
            //if there is no set of tokens, we'll simply return an empty vector
            return vec![];
        };

        // where to start pagination
        // - if we have a from_index, we'll use that - otherwise return ALL tokens
        let start = u128::from(from_index.unwrap_or(U128(0)));

        //iterate through the keys vector
        tokens
            .iter()
            //skip to the index we specified in the start variable
            .skip(start as usize)
            //take the first "limit" elements in the vector. If we didn't specify a limit, return until the last element
            .take(limit.unwrap_or(tokens.len()) as usize)
            //we'll map the token IDs which are strings into Json Tokens
            .map(|token_id| self.json_token(token_id.clone()).unwrap())
            //since we turned the keys into an iterator, we need to turn it back into a vector to return
            .collect()
    }
}