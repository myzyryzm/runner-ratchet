use std::fmt::format;

use crate::*;
use crate::{internal::*, this_contract::CallbacksExt};
use near_sdk::{ext_contract, log, Gas, PromiseResult};

const GAS_FOR_RESOLVE_TRANSFER: Gas = Gas(10_000_000_000_000);
const GAS_FOR_NFT_ON_TRANSFER: Gas = Gas(25_000_000_000_000);
const NFT_CONTRACT: String = "nft.ratchet.testnet".to_string();

pub trait RatchetCore {
    fn update_ratchet(
        &mut self,
        owner_id: AccountId,
        token_id: TokenId,
        experience: u64,
    ) -> Option<JsonRatchet>;

    fn ratchet(&self, token_id: TokenId) -> Option<JsonRatchet>;
}

#[near_bindgen]
impl RatchetCore for Contract {
    fn update_ratchet(
        &mut self,
        owner_id: AccountId,
        token_id: TokenId,
        experience: u64,
    ) -> Option<JsonRatchet> {
        assert_eq!(
            env::predecessor_account_id(),
            env::current_account_id(),
            "Only the contract account can update data."
        );

        if let Some(ratchet) = self.ratchets_by_id.get(&token_id) {
            assert_eq!(
                ratchet.owner_id, owner_id,
                "{} is not the owner of ratchet with id of {}",
                owner_id, token_id
            );
            if let Some(mut metadata) = self.ratchet_metadata_by_id.get(&token_id) {
                metadata.experience = experience;
                self.ratchet_metadata_by_id.insert(&token_id, &metadata);
                Some(JsonRatchet {
                    token_id,
                    owner_id: ratchet.owner_id,
                    metadata,
                })
            } else {
                let msg = format!("Ratchet metadat with id {} does not exist", token_id);
                env::panic_str(&msg);
            }
        } else {
            let msg = format!("Ratchet with id {} does not exist", token_id);
            env::panic_str(&msg);
        }
        None
    }

    /// get the information for a ratchet id
    fn ratchet(&self, token_id: TokenId) -> Option<JsonRatchet> {
        if let Some(ratchet) = self.ratchets_by_id.get(&token_id) {
            // we'll get the metadata for that token
            let metadata = self.ratchet_metadata_by_id.get(&token_id).unwrap();
            Some(JsonRatchet {
                token_id,
                owner_id: ratchet.owner_id,
                metadata,
            })
        } else {
            // if there wasnt a token id in the tokens_by_id collection we return None
            None
        }
    }
}

#[near_bindgen]
impl CallbacksExt for Contract {
    /// this runs after calling the mint function on the ratchet nft contract
    pub fn on_resolve_mint_callback(
        &mut self,
        owner_id: AccountId,
        token_id: TokenId,
        #[callback_result] call_result: Result<(), PromiseError>,
    ) -> bool {
        if call_result.is_err() {
            env::log_str("Unable to create nft, rolling back...");
            let total_cost = (RUNNER_MINT_COST + NFT_MINT_COST) * YOCTO_NEAR;
            Promise::new(owner_id).transfer(total_cost);
            return false;
        }

        let token = Ratchet { owner_id: owner_id };

        if self.ratchets_by_id.contains_key(&token_id) {
            Promise::new(owner_id).transfer(RUNNER_MINT_COST * YOCTO_NEAR);
            env::panic_str("Ratchet with id already exists on runner contract");
        }

        if self.ratchet_per_owner.contains_key(&owner_id) {
            Promise::new(owner_id).transfer(RUNNER_MINT_COST * YOCTO_NEAR);
            env::panic_str("Owner already has a ratchet runner");
        }

        self.ratchets_by_id.insert(&token_id, &token);
        self.ratchet_per_owner.insert(&owner_id, &token_id);

        let metadata = RatchetMetadata {
            nft_token_id: token_id,
            experience: 0,
        };

        self.ratchet_metadata_by_id.insert(&token_id, &metadata);

        true
    }

    /// this runs after attempting to add the ratchet runner data (i.e. the runner id to the nft map)
    pub fn add_to_ratchet_nft_callback(
        &mut self,
        owner_id: AccountId,
        token_id: TokenId,
        #[callback_result] call_result: Result<(), PromiseError>,
    ) -> bool {
        if call_result.is_err() {
            env::log_str("Unable to add to nft contract, rolling back...");
            // todo: need to check to make sure the token still exists (if somehow it was removed then these could fail)
            // remove token from owner
            self.ratchet_per_owner.remove(&owner_id);
            // remove token
            self.ratchets_by_id.remove(&token_id);
            // remove token metadata
            self.ratchet_metadata_by_id.remove(&token_id);
            // transfer funds back to user
            Promise::new(owner_id).transfer(YOCTO_NEAR * RUNNER_MINT_COST);
            return false;
        }
        true
    }
}
