use crate::*;
use crate::{internal::*, this_contract::CallbacksExt};
use near_sdk::{ext_contract, log, Gas, PromiseResult};

const GAS_FOR_RESOLVE_TRANSFER: Gas = Gas(10_000_000_000_000);
const GAS_FOR_NFT_ON_TRANSFER: Gas = Gas(25_000_000_000_000);
const NFT_CONTRACT: String = "nft.ratchet.testnet".to_string();

pub trait RatchetCore {
    fn set_new_owner(&mut self, receiver_id: AccountId, token_id: TokenId);

    fn update(&mut self, owner_id: AccountId, token_id: TokenId);
}

#[near_bindgen]
impl RatchetCore for Contract {
    fn set_new_owner(&mut self, receiver_id: AccountId, token_id: TokenId) {
        let sender_id = env::predecessor_account_id();
        assert_eq!(
            sender_id, NFT_CONTRACT,
            "This method can only be called from {}",
            NFT_CONTRACT
        );
        let owner_id = env::signer_account_id();
        self.internal_transfer(&owner_id, &receiver_id, &token_id);
    }

    fn update(&mut self, owner_id: AccountId, token_id: TokenId, experience: u64) {
        assert_eq!(
            env::predecessor_account_id() == env::current_account_id(),
            "Only the contract account can update data."
        );
    }
}

#[near_bindgen]
impl CallbacksExt for Contract {
    pub fn add_to_nft_callback(
        &mut self,
        owner_id: AccountId,
        token_id: TokenId,
        #[callback_result] call_result: Result<(), PromiseError>,
    ) -> bool {
        if call_result.is_err() {
            env::log_str("Unable to add to nft contract, rolling back...");
            // todo: need to check to make sure the token still exists (if somehow it was removed then these could fail)
            // remove token from owner
            self.internal_remove_token_from_owner(&owner_id, &token_id);
            // remove token
            self.tokens_by_id.remove(&token_id);
            // remove token metadata
            self.token_metadata_by_id.remove(&token_id);
            // transfer funds back to user
            Promise::new(owner_id).transfer(YOCTO_NEAR * MINT_COST);
            return false;
        }
        true
    }

    pub fn on_resolve_mint_callback(
        &mut self,
        owner_id: AccountId,
        token_id: TokenId,
        #[callback_result] call_result: Result<(), PromiseError>,
    ) -> bool {
        if call_result.is_err() {
            env::log_str("Unable to create nft, rolling back...");
            Promise::new(owner_id).transfer(YOCTO_NEAR * NFT_MINT_COST);
            return false;
        }
        let token = Ratchet { owner_id: owner_id };

        assert!(
            self.ratchets_by_id.insert(&token_id, &token).is_none(),
            "Token already exists"
        );

        let metadata = RatchetMetadata {
            nft_token_id: token_id,
            experience: 0,
        };

        self.token_metadata_by_id.insert(&token_id, &metadata);

        self.internal_add_token_to_owner(&token.owner_id, &token_id);
        true
    }
}
