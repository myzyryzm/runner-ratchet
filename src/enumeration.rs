use crate::*;

#[near_bindgen]
impl Contract {
    pub fn nft_total_supply(&self) -> U128 {
        U128(self.token_metadata_by_id.len() as u128)
    }

    // pub fn nft_tokens(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<JsonToken> {
    //     let start = u128::from(from_index.unwrap_or(U128(0)));

    //     self.token_metadata_by_id.keys().skip(start as usize).take()
    // }
}
