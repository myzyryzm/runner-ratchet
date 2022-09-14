use crate::*;
use near_sdk::Gas;

pub const YOCTO_NEAR: u128 = 1_000_000_000_000_000_000_000_000;
pub const RUNNER_MINT_COST: u128 = 0; // cost to create a character on this game
pub const NFT_MINT_COST: u128 = 1; // cost to create the NFT version of character

const GAS_FOR_CREATE_NFT: Gas = Gas(50_000_000_000_000);
const GAS_FOR_RESOLVE_CREATE: Gas = Gas(50_000_000_000_000);

#[near_bindgen]
impl Contract {
    // This method is for creating the ratchet nft character and then for creating the character on the game
    #[payable]
    pub fn full_mint_ratchet(&mut self, token_id: TokenId, receiver_id: AccountId) {
        assert!(
            self.ratchets_by_id.contains_key(&token_id) == false,
            "Ratchet with id {} already exists",
            token_id
        );

        assert!(
            self.ratchet_per_owner.contains_key(&receiver_id) == false,
            "Account {} already has a character",
            receiver_id
        );

        let attached = env::attached_deposit();
        let base_cost = RUNNER_MINT_COST + NFT_MINT_COST;
        let total_cost = base_cost * YOCTO_NEAR;

        assert_eq!(
            attached, total_cost,
            "Must attach {} Near to create character",
            base_cost
        );

        let promise = nft_ratchet::ext("nft.ratchet.testnet".parse().unwrap())
            .with_static_gas(GAS_FOR_CREATE_NFT)
            .with_attached_deposit(NFT_MINT_COST * YOCTO_NEAR)
            .nft_mint_from_game(env::current_account_id(), token_id, receiver_id);

        return promise.then(
            Self::ext(env::current_account_id())
                .with_static_gas(GAS_FOR_RESOLVE_CREATE)
                .on_resolve_mint_callback(receiver_id, token_id),
        );
    }

    // this is for creating the ratchet character to be used on the actual game; right now it costs money to add the character to the game but that is still up in the air
    #[payable]
    pub fn mint_ratchet(
        &mut self,
        token_id: TokenId,
        nft_token_id: TokenId,
        receiver_id: AccountId,
        nft_contract: Option<String>,
    ) {
        let attached_deposit = env::attached_deposit();
        let required_cost = YOCTO_NEAR * RUNNER_MINT_COST;

        // assert_one_yocto();

        assert!(
            required_cost == attached_deposit,
            "Must attach {} NEAR to purchase",
            RUNNER_MINT_COST,
        );

        let token = Ratchet {
            owner_id: receiver_id,
        };

        // insert the token ID and token strct and make sure that the token doesn't exist
        assert!(
            self.ratchets_by_id.insert(&token_id, &token).is_none(),
            "Token already exists"
        );

        assert!(
            self.ratchet_per_owner
                .insert(&owner_id, &token_id)
                .is_none(),
            "{} already has a ratchet character",
            owner_id
        );

        let metadata = RatchetMetadata {
            nft_token_id: nft_token_id,
            experience: 0,
        };

        self.ratchet_metadata_by_id.insert(&token_id, &metadata);

        let tar_contract = nft_contract.unwrap_or("nft.ratchet.testnet".parse().unwrap());

        let promise = nft_ratchet::ext(tar_contract)
            .with_static_gas(GAS_FOR_CREATE_NFT)
            .add_to_nft(nft_token_id.clone(), token_id.clone());
        return promise.then(
            Self::ext(env::current_account_id())
                .with_static_gas(GAS_FOR_RESOLVE_CREATE)
                .add_to_ratchet_nft_callback(token.owner_id, token_id),
        );
    }
}
