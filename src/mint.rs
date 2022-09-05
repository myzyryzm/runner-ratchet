use crate::*;
use near_sdk::Gas;

const POSSIBLE_CONTRACTS: &[&str] = &["runner.ratchet.testnet"];
pub const YOCTO_NEAR: u128 = 1_000_000_000_000_000_000_000_000;
pub const MINT_COST: u128 = 0.1; // cost to create a character
pub const NFT_MINT_COST: u128 = 1;

const GAS_FOR_RESOLVE_TRANSFER: Gas = Gas(10_000_000_000_000);
const GAS_FOR_NFT_ON_TRANSFER: Gas = Gas(25_000_000_000_000);
const GAS_FOR_CREATE_NFT: Gas = Gas(50_000_000_000_000);
const GAS_FOR_RESOLVE_CREATE: Gas = Gas(50_000_000_000_000);

#[near_bindgen]
impl Contract {
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

        let promise = nft_ratchet::ext("nft.ratchet.testnet".parse().unwrap())
            .with_static_gas(GAS_FOR_CREATE_NFT)
            .with_attached_deposit(env::attached_deposit())
            .nft_mint_from_game(env::current_account_id(), token_id, receiver_id);

        return promise.then(
            Self::ext(env::current_account_id())
                .with_static_gas(GAS_FOR_RESOLVE_CREATE)
                .on_resolve_mint_callback(receiver_id, token_id),
        );
    }

    #[payable]
    pub fn nft_mint_ratchet(
        &mut self,
        token_id: TokenId,
        mut metadata: TokenMetadata,
        receiver_id: AccountId,
    ) {
        let attached_deposit = env::attached_deposit();
        let required_cost = YOCTO_NEAR * MINT_COST;

        assert!(
            required_cost == attached_deposit,
            "Must attach {} NEAR to purchase",
            MINT_COST,
        );

        let token = Token {
            owner_id: receiver_id,
        };

        // insert the token ID and token strct and make sure that the token doesn't exist
        assert!(
            self.tokens_by_id.insert(&token_id, &token).is_none(),
            "Token already exists"
        );

        self.token_metadata_by_id.insert(&token_id, &metadata);

        self.internal_add_token_to_owner(&token.owner_id, &token_id);

        let promise = nft_ratchet::ext("nft.ratchet.testnet".parse().unwrap())
            .with_static_gas(GAS_FOR_NFT_ON_TRANSFER)
            .add_to_nft(metadata.nft_token_id.clone(), token_id.clone());
        return promise.then(
            Self::ext(env::current_account_id())
                .with_static_gas(GAS_FOR_RESOLVE_TRANSFER)
                .add_to_nft_callback(token.owner_id, token_id),
        );
    }
}
