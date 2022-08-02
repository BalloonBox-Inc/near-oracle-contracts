<p align="center">
  <a href="https://near.org/">
    <img alt="NearMonotoneWhite" src="https://github.com/BalloonBox-Inc/near-oracle-contracts/blob/dev/images/monotone-white-bg.png" width="1300" />
  </a>
</p>

# PUBLIC METHODS

#### About :spiral_notepad:
The MFT-minter smart contract contains numerous public methods or functions, callable from outside of the contract. This documentations lists a few noteworthy methods, namely some of the ones invoked by the dApp as the frontend interacts with the smart contract. Methods can be of two types:
 - **calls: (cost gas)** these methods alter the contract state, i.e., they're state handlers
 - **views: (gasless)** these methods are view-only and are used to query the contract state without changing it

#### Help Us :handshake:
Have you spotted a mistake in our NEARoracle docs? Help us improve it by [letting us know](https://www.balloonbox.io/contact).

## Function Calls
List of state-handling functions.
```bash
    #mint a token as an NFT and returns a struct indicating
    #whether the minting operation was successful
    pub fn nft_mint(
        &mut self,
        token_id: TokenId,
        metadata: TokenMetadata,
        receiver_id: AccountId,
    ) -> MintOutcome { ... }
```
> Find the complete code of the *nft_mint()* function in the file [`./contract-nft/src/mint.rs`](src/mint.rs).

## View Calls
List of view-only functions.
```bash
#pass in a token_id (NFT) and returns the wallet address that owns it
pub fn whose_token(
    &self,
    token_id: TokenId
    ) -> Option<AccountId> { ... }

#returns the total count of NFTs stored in the smart contract
pub fn nft_total_supply(
    &self
    ) -> U128 { ... }

#returns a vector of token structs stored in the smart contract
#pass in optional lower and upper vector indeces
pub fn nft_tokens(
    &self,
    from_index: Option<U128>,
    limit: Option<u64>
    ) -> Vec<JsonToken> { ... }

#pass in a Near account and returns the count of NFTs owned
pub fn nft_supply_for_owner(
    &self,
    account_id: AccountId
    ) -> U128 { ... }

#pass in a Near account and returns a vector of owned token structs
#pass in optional lower and upper vector indeces
    pub fn nft_tokens_for_owner(
        &self,
        account_id: AccountId,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<JsonToken> { ... }

#returns basic descriptive metadata about the smart contract
    pub fn nft_metadata(
        &self,
    ) -> NFTContractMetadata { ... }
```
> These view-only functions are stored in either one of the Rust files: [`./contract-nft/src/enumerate.rs`](src/enumerate.rs) or [`./contract-nft/src/metadata.rs`](src/metadata.rs).
> 
> N.B. The above documentation does not contain the function logic. We omitted it intentionally for readability sake, replacing it with the `{ ... }` placeholder. If you want to see the full source code, please consult the .rs files where the functions are stored.