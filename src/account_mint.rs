use crate::*;
use near_sdk::Gas;

pub const ACCOUNT_MINT_COST: u128 = 0.1; // cost to create a account for this game

#[near_bindgen]
impl Contract {
    /// This method is for creating the account that will keep track of the user's general progress
    #[payable]
    pub fn mint_account(&mut self, account_id: AccountId) {
        assert!(
            self.account_metadata_by_id.contains_key(&account_id) == false,
            "Account with id {} already exists",
            account_id
        );

        let attached = env::attached_deposit();
        let total_cost = ACCOUNT_MINT_COST * YOCTO_NEAR;

        assert_eq!(
            attached, total_cost,
            "Must attach {} Near to create character",
            base_cost
        );
        let metadata = AccountMetadata { progress: 0 };
        self.account_metadata_by_id.insert(&account_id, &metadata);
    }
}
