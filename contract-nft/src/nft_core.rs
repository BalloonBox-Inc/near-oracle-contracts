use crate::*;
use near_sdk::{ext_contract, log, Gas, PromiseResult};

const GAS_FOR_RESOLVE_TRANSFER: Gas = Gas(10_000_000_000_000);
const GAS_FOR_NFT_ON_TRANSFER: Gas = Gas(25_000_000_000_000);

pub trait NonFungibleTokenCore {
    /*
        Trait containing the primary functions to transfer an NFT from user A to user B
     */
    //get information about the NFT token passed in
    fn json_token(&self, token_id: TokenId) -> Option<JsonToken>;

    //transfers an NFT to a receiver ID
    fn nft_transfer(
        &mut self,
        receiver_id: AccountId,
        token_id: TokenId,
        approval_id: Option<u64>,
        memo: Option<String>,
    );

    //transfers an NFT to a receiver and calls a function on the receiver ID's contract
    //Returns `true` if the token was transferred from the sender's account.
    fn nft_transfer_call(
        &mut self,
        receiver_id: AccountId,
        token_id: TokenId,
        memo: Option<String>,
        approval_id: Option<u64>,
        msg: String,
    ) -> PromiseOrValue<bool>;
}



#[ext_contract(ext_non_fungible_token_receiver)]
trait NonFungibleTokenReceiver {
    /*
        Method stored on the receiver contract that is called via cross contract call when nft_transfer_call is called
        Returns `true` if the token should be returned back to the sender.
     */
    fn nft_on_transfer(
        &mut self,
        sender_id: AccountId,
        previous_owner_id: AccountId,
        token_id: TokenId,
        msg: String,
    );
}


#[ext_contract(ext_self)]
trait NonFungibleTokenResolver {
    /*
        resolves the promise of the cross contract call to the receiver contract
        this is stored on THIS contract and is meant to analyze what happened in the cross contract call when nft_on_transfer was called
        as part of the nft_transfer_call method
    */
    fn nft_resolve_transfer(
        &mut self,
        owner_id: AccountId,
        receiver_id: AccountId,
        token_id: TokenId,
        approved_account_ids: HashMap<AccountId, u64>,
    ) -> bool;
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
                approved_account_ids: token.approved_account_ids,
                royalty: token.royalty,
            })
        //if there is no token ID in the token_by_id_collection, then return None
        } else {
            None
        }
    }


    //implementation of the nft_transfer method. 
    //This transfers the NFT from the current owner to the receiver.
    #[payable]
    fn nft_transfer(
        &mut self,
        receiver_id: AccountId,
        token_id: TokenId,
        approval_id: Option<u64>,
        memo: Option<String>,
    ) {
        //assert that the user attached exactly 1 yoctoNEAR.
        //This is for security and so that the user will be redirected to the NEAR wallet.
        assert_one_yocto();
        //get the sender to transfer the token from the sender to the receiver
        let sender_id = env::predecessor_account_id();

        //call the internal transfer method
        let previous_token = self.internal_transfer(
            &sender_id,
            &receiver_id,
            &token_id,
            approval_id,
            memo,
        );

        //we refund the owner for releasing the storage used up by the approved account IDs
        refund_approved_account_ids(
            previous_token.owner_id.clone(),
            &previous_token.approved_account_ids,
        );
    }



    //implementation of the transfer call method.
    //This will transfer the NFT and call a method on the reciver_id contract
    #[payable]
    fn nft_transfer_call(
        &mut self,
        receiver_id: AccountId,
        token_id: TokenId,
        memo: Option<String>,
        approval_id: Option<u64>,
        msg: String,
    ) -> PromiseOrValue<bool> {
        //assert that the user attached exactly 1 yoctoNEAR.
        assert_one_yocto();
        //get the sender to transfer the token from the sender to the receiver
        let sender_id = env::predecessor_account_id();
        
        //transfer the token and get the previous token object
        let previous_token = self.internal_transfer(
            &sender_id,
            &receiver_id,
            &token_id,
            approval_id,
            memo,
        );

        //Initiating receiver's call and the callback
        ext_non_fungible_token_receiver::ext(receiver_id.clone())
            .with_static_gas(GAS_FOR_NFT_ON_TRANSFER)
            .nft_on_transfer(
                sender_id,
                previous_token.owner_id.clone(),
                token_id.clone(),
                msg
            )
        //we then resolve the promise and call nft_resolve_transfer on our own contract
        .then(
            Self::ext(env::current_account_id())
                .with_static_gas(GAS_FOR_RESOLVE_TRANSFER)
                .nft_resolve_transfer(
                    previous_token.owner_id,
                    receiver_id,
                    token_id,
                    previous_token.approved_account_ids,
                )
        ).into()
    }
}



#[near_bindgen]
impl NonFungibleTokenResolver for Contract {
    //resolves the cross contract call when calling nft_on_transfer in the nft_transfer_call method
    //returns true if the token was successfully transferred to the receiver_id
    #[private]
    fn nft_resolve_transfer(
        &mut self,
        owner_id: AccountId,
        receiver_id: AccountId,
        token_id: TokenId,
        approved_account_ids: HashMap<AccountId, u64>,
    ) -> bool {
        //returns a boolean indicating whether the token should be returned back to its sender
        if let PromiseResult::Successful(value) = env::promise_result(0) {
            //As per the standard, the nft_on_transfer tells us whether we should return the token to it's owner or not
            if let Ok(return_token) = near_sdk::serde_json::from_slice::<bool>(&value) {
                //if we don't need to return the token, we simply return true meaning everything went fine 
                if !return_token {
                    /* 
                        since we've already transferred the token and nft_on_transfer returned false, we don't have to 
                        revert the original transfer and thus we can just return true since nothing went wrong.
                    */
                    //case 1: the NFT transfer went through: all is well! All remains to do is refunding
                    //the old NFT owner for freeing storage space since we reset the approved_account_ids HashMap
                    refund_approved_account_ids(owner_id, &approved_account_ids);
                    return true;
                }
            }
        }

        //get the token object if there is some token object
        let mut token = if let Some(token) = self.token_by_id.get(&token_id) {
            if token.owner_id != receiver_id {
                //the token is not owned by the receiver anymore. Can't return it.
                refund_approved_account_ids(owner_id, &approved_account_ids);
                return true;
            }
            token
        //if there isn't a token object, it was burned and so we return true
        } else {
            refund_approved_account_ids(owner_id, &approved_account_ids);
            return true;
        };

        //if at the end, we haven't returned true, that means that we should return the token to it's original owner
        log!("Return {} from @{} to @{}", token_id, receiver_id, owner_id);
        //remove the token from the receiver
        self.internal_remove_token_from_owner(&receiver_id, &token_id);
        //add the token to the original owner
        self.internal_add_token_to_owner(&owner_id, &token_id);

        //change the token struct's owner to be the original owner 
        token.owner_id =  owner_id;

        //we refund the receiver any approved account IDs that they may have set on the token
        refund_approved_account_ids(receiver_id, &token.approved_account_ids);
        //reset the approved account IDs to what they were before the transfer
        token.approved_account_ids = approved_account_ids;

        //we inset the token back into the tokens_by_id collection
        self.token_by_id.insert(&token_id, &token);

        //return false
        false
    }
}