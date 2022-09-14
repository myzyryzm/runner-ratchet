use crate::*;
pub type TokenId = String;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ContractMetadata {
    // required, essentially a version like "nft-1.0.0"
    pub spec: String,
    pub name: String,
    pub symbol: String,
}

pub trait NonFungibleTokenMetadata {
    //view call for returning the contract metadata
    fn nft_metadata(&self) -> ContractMetadata;
}

#[near_bindgen]
impl NonFungibleTokenMetadata for Contract {
    fn nft_metadata(&self) -> ContractMetadata {
        self.metadata.get().unwrap()
    }
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct AccountMetadata {
    pub progress: u64, // game specific progress
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Ratchet {
    // owner of token
    pub owner_id: AccountId,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct RatchetMetadata {
    pub nft_token_id: TokenId, // id of ratchet on nft contract
    pub experience: u64,       // game specific experience
}

//The Json token is what will be returned from view calls.
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct JsonRatchet {
    // token id
    pub token_id: TokenId,
    // owner of the token
    pub owner_id: AccountId,
    // token metadata
    pub metadata: RatchetMetadata,
}
