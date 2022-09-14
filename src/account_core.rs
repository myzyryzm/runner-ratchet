use std::fmt::format;

use crate::*;
use crate::{internal::*, this_contract::CallbacksExt};
use near_sdk::{ext_contract, log, Gas, PromiseResult};

const GAS_FOR_RESOLVE_TRANSFER: Gas = Gas(10_000_000_000_000);
const GAS_FOR_NFT_ON_TRANSFER: Gas = Gas(25_000_000_000_000);
const NFT_CONTRACT: String = "nft.ratchet.testnet".to_string();

pub trait AccountCore {
    fn update_account(&mut self, owner_id: AccountId, progress: u64) -> Option<JsonAccount>;

    fn account(&self, token_id: TokenId) -> Option<JsonAccount>;
}

#[near_bindgen]
impl AccountCore for Contract {
    fn update_account(&mut self, owner_id: AccountId, progress: u64) -> Option<JsonAccount> {
        assert_eq!(
            env::predecessor_account_id(),
            env::current_account_id(),
            "Only the contract account can update data."
        );

        if let Some(mut metadata) = self.account_metadata_by_id.get(&owner_id) {
            metadata.progress = progress;
            self.account_metadata_by_id.insert(&owner_id, &metadata);
            Some(JsonAccount { owner_id, metadata })
        } else {
            let msg = format!("Account with id {} does not exist", owner_id);
            env::panic_str(&msg);
        }
        None
    }

    /// get the information for a ratchet id
    fn account(&self, owner_id: AccountId) -> Option<JsonAccount> {
        if let Some(metadata) = self.account_metadata_by_id.get(&owner_id) {
            Some(JsonAccount { owner_id, metadata })
        } else {
            // if there wasnt a token id in the tokens_by_id collection we return None
            None
        }
    }
}
