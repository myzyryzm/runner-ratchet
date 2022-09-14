use crate::*;

#[near_bindgen]
impl Contract {
    // get number of ratchets
    pub fn ratchet_total_supply(&self) -> U128 {
        U128(self.ratchet_metadata_by_id.len() as u128)
    }

    //Query for ratchets on the contract regardless of the owner using pagination
    pub fn ratchets(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<JsonRatchet> {
        let start = u128::from(from_index.unwrap_or(U128(0)));

        self.ratchet_metadata_by_id
            .keys()
            .skip(start as usize)
            .take(limit.unwrap_or(50) as usize)
            .map(|token_id| self.ratchet(token_id.clone()).unwrap())
            .collect()
    }

    // get ratchet belonging to owner
    pub fn ratchets_for_owner(&self, account_id: AccountId) -> Option<JsonRatchet> {
        if let Some(ratchet_id) = self.ratchet_per_owner.get(&account_id) {
            let metadata = self.ratchet_metadata_by_id.get(&ratchet_id).unwrap();
            Some(JsonRatchet {
                token_id,
                owner_id: ratchet.owner_id,
                metadata,
            })
        } else {
            None
        }
    }
}
