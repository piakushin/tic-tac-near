use near_contract_standards::fungible_token::receiver::FungibleTokenReceiver;
use near_sdk::{json_types::U128, near_bindgen, AccountId, Promise, PromiseOrValue};

use crate::{Contract, ContractExt};

#[near_bindgen]
impl FungibleTokenReceiver for Contract {
    fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128> {
        todo!()
    }
}
