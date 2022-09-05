use crate::*;
use near_sdk::ext_contract;

//
#[ext_contract(this_contract)]
trait Callbacks {
    fn add_to_nft_callback(&mut self, owner_id: AccountId, token_id: TokenId) -> bool;

    fn on_resolve_mint_callback(&mut self, owner_id: AccountId, token_id: TokenId) -> bool;
}

#[ext_contract(nft_ratchet)]
trait NFTRatchet {
    fn add_to_nft(&mut self, token_id: TokenId, game_token_id: TokenId);

    fn nft_mint_from_game(&mut self, contract_id: String, ratchet_id: TokenId, owner_id: String);
}
