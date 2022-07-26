use crate::*;
use near_sdk::CryptoHash;
use std::mem::size_of;


/*
This .rs files contains all 'internal methods' for the contract, 
meaning that all he functions declared here 
are called internally by the contract, but 
they can't be called externally from the CLI.
There are 2 types of methods:
(1) gasless methods: that do NOT act directly on the main singleton and DON'T modify the contract state
(2) gas methods: that do act directly on the singleton and DON'T 
 */


// ------------------------------- //
//         gasless methods         //
// ------------------------------- //

//calcualtes how many bytes of storage is taken up by each approved account id
pub (crate) fn bytes_for_approved_account_id(account_id: &AccountId) -> u64 {
    //The extra 4 bytes are coming from Borsh serialization to store the length of the string.
    account_id.as_str().len() as u64 + 4 + size_of::<u64>() as u64
}

//refund the cost for storage taken up by the approved account IDs saved under a given account
//and send the funds to the given account
pub (crate) fn refund_approved_account_ids_iter<'a, I>(
    account_id: AccountId,
    //the approved account IDs must be passed in as an iterator "I"
    approved_account_ids: I, 
) -> Promise where I: Iterator<Item = &'a AccountId>,
{
    //get the storage total by going through and summing all the bytes for each approved account IDs
    let storage_released: u64 = approved_account_ids.map(bytes_for_approved_account_id).sum();
    //transfer into the account the storage that is released
    Promise::new(account_id).transfer(Balance::from(storage_released) * env::storage_byte_cost())
}

//takes a map of approved account IDs and refund the storage cost to a given account
pub (crate) fn refund_approved_account_ids(
    account_id: AccountId,
    approved_account_ids: &HashMap<AccountId, u64>,
) -> Promise {
    //call the function "refund_Approved_account_ids_iter" and pass the approved account IDs as keys
    refund_approved_account_ids_iter(account_id, approved_account_ids.keys())
}

//used to generate a unique prefix in our storage collections (this is to avoid data collisions)
pub(crate) fn hash_account_id(account_id: &AccountId) -> CryptoHash {
    //get the default hash
    let mut hash = CryptoHash::default();
    //we hash the account ID and return it
    hash.copy_from_slice(&env::sha256(account_id.as_bytes()));
    hash
}

pub (crate) fn assert_one_yocto() {
    assert_eq!(env::attached_deposit(), 1, "Required attached deposit of exactly 1 yoctoNEAR")
}

pub (crate) fn assert_at_least_one_yocto() {
    assert!(env::attached_deposit() >= 1, "Requires attached de[osit of at least 1 yoctoNEAR")
}

//refund the initial deposit based on the amount of storage that was used up
pub(crate) fn refund_deposit(storage_used: u64) {
    //get how much it would cost to store the information
    let required_cost = env::storage_byte_cost() * Balance::from(storage_used);
    //get the attached deposit
    let attached_deposit = env::attached_deposit();

    //make sure that the attached deposit is greater than or equal to the required cost
    assert!(
        required_cost <= attached_deposit,
        "Must attach {} yoctoNEAR to cover storage",
        required_cost,
    );

    //get the refund amount from the attached deposit - required cost
    let refund = attached_deposit - required_cost;

    //if the refund is greater than 1 yocto NEAR, we refund the predecessor that amount
    if refund > 1 {
        Promise::new(env::predecessor_account_id()).transfer(refund);
    }
}

// ------------------------------- //
//           gas methods           //
// ------------------------------- //
// These methods cost a gas fee because they change the state of the contract and thus 
// incur into a fee for storing and altering data on blockchain. Since they change 
// contract state, then they take in "&mut self" as one of their parameters and 
// therefore must be implemented as methods on the "Contract" struct
// Gas methods == change methods

impl Contract {
    //add a token to the set of tokens an owner has
    pub(crate) fn internal_add_token_to_owner(
        &mut self,
        account_id: &AccountId,
        token_id: &TokenId,
    ) {
        //get the set of tokens for the given account
        let mut tokens_set = self.tokens_per_owner.get(account_id).unwrap_or_else(|| {
            //if the account doesn't have any tokens, we create a new unordered set
            UnorderedSet::new(
                StorageKey::TokenPerOwnerInner {
                    //we get a new unique prefix for the collection
                    account_id_hash: hash_account_id(&account_id),
                }
                .try_to_vec()
                .unwrap(),
            )
        });

        //we insert the token ID into the set
        tokens_set.insert(token_id);

        //we insert that set for the given account ID.
        self.tokens_per_owner.insert(account_id, &tokens_set);
    }

    pub(crate) fn internal_remove_token_from_owner(
        &mut self, account_id: &AccountId, token_id: &TokenId) {
            // get the set of tokens that the owner has
            let mut tokens_set = self
                .tokens_per_owner
                .get(account_id)
                //if there is no set of tokens for the owner, we panic with the following message:
                .expect("Token should be owned by the sender");
                
            // remove the the token_id from the set of tokens
            tokens_set.remove(token_id);
            // if the token set is now empty, we remove the owner from the tokens_per_owner collection
            if tokens_set.is_empty() {
                self.tokens_per_owner.remove(account_id); 
            } else {
            //if the token set is not empty, we simply insert it back for the account ID.
            self.tokens_per_owner.insert(account_id, &tokens_set);
            }
        }


    //transfers the NFT to the receiver_id (internal method and can't be called directly via CLI).
    pub (crate) fn internal_transfer(
        &mut self,
        sender_id: &AccountId,
        receiver_id: &AccountId,
        token_id: &TokenId,
        //we introduce an approval ID so that people with that approval ID can transfer the token
        approval_id: Option<u64>,
        memo: Option<String>,
    ) -> Token {
        //get the token object by passing the token_id
        let token = self.token_by_id.get(&token_id).expect("No token");

        //if the sender doesn't equal the owner, we panic
        if sender_id != &token.owner_id { 
            //if the token's approved account IDs doesn't contain the sender, we panic
            if !token.approved_account_ids.contains_key(sender_id) {
                env::panic_str("Unauthorized");
            }
            
            //If they included an approval_id, check if the sender's actual approval_id is the same as the one included
            if let Some(enforced_approval_id) =  approval_id {
                //get the actual approval ID
                let actual_approval_id = token.approved_account_ids.get(sender_id)
                //if the sender isn't in the map we panic 
                .expect("Sender is not approved account");

                //make sure that the actual approval ID is the same as the one provided
                assert_eq!(
                    actual_approval_id, &enforced_approval_id,
                    "The actual approval_id {} is different from the given approval_id {}",
                    actual_approval_id, enforced_approval_id,
                );
            }
        }

        //make sure that the sender isn't sending the token to themselves
        assert_ne!(&token.owner_id, receiver_id, "The token owner and the receiver should be different");

        //remove the token from it's current owner's set
        self.internal_remove_token_from_owner(&token.owner_id, token_id);
        //add the token to the receiver_id's set
        self.internal_add_token_to_owner(receiver_id, token_id);

        //create a new token struct 
        let new_token = Token {
            owner_id : receiver_id.clone(),
            //reset the approval account IDs
            approved_account_ids: Default::default(),
            next_approval_id: token.next_approval_id,
        };

        //insert that new token id into the tokens_by_id, replacing the old entry
        self.token_by_id.insert(token_id, &new_token);
        
        //if there was some memo attached, then log it
        if let Some(memo) =  memo {
            env::log_str(&format!("Memo: {}", memo).to_string());
        }
        //return the previous token object that was transferred
        token
    }
}