use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    json_types::U128,
    near_bindgen, AccountId,
};
use serde::Serialize;

#[near_bindgen]
#[derive(Debug, BorshDeserialize, BorshSerialize, Serialize, Clone)]
pub struct Player {
    account: AccountId,
    deposit: U128,
    pub stream: Option<String>,
}

impl Player {
    pub fn new(account: AccountId, deposit: U128) -> Self {
        Self {
            account,
            deposit,
            stream: None,
        }
    }

    pub fn account(&self) -> &AccountId {
        &self.account
    }

    pub fn deposit(&self) -> U128 {
        self.deposit
    }

    pub fn stream(&self) -> Option<&String> {
        self.stream.as_ref()
    }
}
