use near_contract_standards::fungible_token::receiver::FungibleTokenReceiver;
use near_sdk::{json_types::U128, log, near_bindgen, AccountId, PromiseOrValue};

use crate::{field::Field, player::Player, Contract, ContractExt};

#[near_bindgen]
impl FungibleTokenReceiver for Contract {
    fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128> {
        log!(
            "received {:?} tokens from {} with msg: {}",
            amount,
            sender_id,
            msg
        );
        assert!(self.field.is_none(), "Game already started");
        if self.first.is_none() {
            self.first = Some(Player::new(sender_id, amount));
            log!("first player registered: {sender_id} with deposit: {amount}");
        } else if self.second.is_none() {
            assert!(
                self.first.as_ref().unwrap().deposit() == amount,
                "deposit should be: {amount:?}"
            );
            self.second = Some(Player::new(sender_id, amount));
            log!("second player registered: {sender_id} with deposit: {amount}, game started!");

            log!("TODO: create streams");

            self.field = Some(Field::new());
        } else {
            panic!("all players are in, registration closed");
        }
        PromiseOrValue::Value(U128::from(0))
    }
}
