use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    near_bindgen, AccountId,
};

#[near_bindgen]
#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct Player {
    account: AccountId,
}

impl Player {
    pub fn new(account: AccountId) -> Self {
        Self { account }
    }

    pub fn account(&self) -> &AccountId {
        &self.account
    }
}
