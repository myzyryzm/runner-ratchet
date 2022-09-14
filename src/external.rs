use crate::*;
use near_sdk::ext_contract;

/// Callbacks for this contract
#[ext_contract(this_contract)]
trait Callbacks {
    fn add_to_ratchet_nft_callback(&mut self, owner_id: AccountId, token_id: TokenId) -> bool;

    fn on_resolve_mint_callback(&mut self, owner_id: AccountId, token_id: TokenId) -> bool;
}

/// Functions that will be run on nft ratchet contract
#[ext_contract(nft_ratchet)]
trait NFTRatchet {
    fn add_to_nft(&mut self, token_id: TokenId, game_token_id: TokenId);

    fn nft_mint_from_game(&mut self, contract_id: String, ratchet_id: TokenId, owner_id: AccountId);
}
