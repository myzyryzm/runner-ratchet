use crate::*;

#[near_bindgen]
impl Contract {
    // get number of ratchets
    pub fn account_total_supply(&self) -> U128 {
        U128(self.account_metadata_by_id.len() as u128)
    }

    //Query for ratchets on the contract regardless of the owner using pagination
    pub fn accounts(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<JsonAccount> {
        let start = u128::from(from_index.unwrap_or(U128(0)));

        self.account_metadata_by_id
            .keys()
            .skip(start as usize)
            .take(limit.unwrap_or(50) as usize)
            .map(|owner_id| self.account(owner_id.clone()).unwrap())
            .collect()
    }
}
