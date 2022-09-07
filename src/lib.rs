use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::json_types::{Base64VecU8, U128};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, near_bindgen, AccountId, Balance, CryptoHash, PanicOnDefault, Promise, PromiseOrValue,
};

pub use crate::enumeration::*;
pub use crate::external::*;
pub use crate::internal::*;
pub use crate::metadata::*;
pub use crate::mint::*;
pub use crate::ratchet_core::*;

mod enumeration;
mod external;
mod internal;
mod metadata;
mod mint;
mod ratchet_core;

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    // contract owner
    pub owner_id: AccountId,
    // keeps track of the metadata for the contract
    pub metadata: LazyOption<NFTContractMetadata>,
    // keeps track of the metadata (e.g. the game progress) for each account
    pub account_metadata_by_id: LookupMap<AccountId, AccountMetadata>,

    // keeps track of all the ratchet tokens per id
    pub ratchet_per_owner: LookupMap<AccountId, TokenId>,
    // keeps track of the token struct for a give tokenId
    pub ratchets_by_id: LookupMap<TokenId, Ratchet>,
    // keeps track of the ratchet metadata for a given token ID
    pub ratchet_metadata_by_id: UnorderedMap<TokenId, RatchetMetadata>,
}

/// Helper structure for keys of the persistent collections.
#[derive(BorshSerialize)]
pub enum StorageKey {
    NFTContractMetadata,
    AccountMetadataById,
    RatchetPerOwner,
    RatchetsById,
    RatchetMetadataById,
}

// Implement the contract structure
#[near_bindgen]
impl Contract {
    /*
        initialization function (can only be called once).
        this initializes the contract with default metadata so the
        user doesn't have to manually type metadata.
    */
    #[init]
    pub fn new_default_meta(owner_id: AccountId) -> Self {
        let this = Self {
            owner_id,
            metadata: NFTContractMetadata {
                spec: "runner-1.0.0".to_string(),
                name: " Ratchet Runner Contract".to_string(),
                symbol: "runner".to_string(),
            },
            account_metadata_by_id: LookupMap::new(
                StorageKey::AccountMetadataById.try_to_vec().unwrap(),
            ),
            ratchet_per_owner: LookupMap::new(StorageKey::RatchetPerOwner.try_to_vec().unwrap()),
            ratchets_by_id: LookupMap::new(StorageKey::RatchetsById.try_to_vec().unwrap()),
            ratchet_metadata_by_id: UnorderedMap::new(
                StorageKey::RatchetMetadataById.try_to_vec().unwrap(),
            ),
        };
        this
    }

    /*
        initialization function (can only be called once).
        this initializes the contract with metadata that was passed in and
        the owner_id.
    */
    #[init]
    pub fn new(owner_id: AccountId, metadata: NFTContractMetadata) -> Self {
        let this = Self {
            owner_id,
            metadata: LazyOption::new(
                StorageKey::NFTContractMetadata.try_to_vec().unwrap(),
                Some(&metadata),
            ),
            account_metadata_by_id: LookupMap::new(
                StorageKey::AccountMetadataById.try_to_vec().unwrap(),
            ),
            ratchet_per_owner: LookupMap::new(StorageKey::RatchetPerOwner.try_to_vec().unwrap()),
            ratchets_by_id: LookupMap::new(StorageKey::RatchetsById.try_to_vec().unwrap()),
            ratchet_metadata_by_id: UnorderedMap::new(
                StorageKey::RatchetMetadataById.try_to_vec().unwrap(),
            ),
        };
        this
    }
}
