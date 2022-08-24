use near_contract_standards::fungible_token::receiver::FungibleTokenReceiver;
use near_sdk::{env, json_types::U128, log, near_bindgen, AccountId, Gas, PromiseOrValue};

use crate::{
    external::{streaming_roketo::streaming_roketo, wrap_near::wrap, TGAS},
    field::Field,
    player::Player,
    Contract, ContractExt,
};

#[near_bindgen]
impl FungibleTokenReceiver for Contract {
    fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128> {
        log!(
            "received {:?} tokens from {} with: {}",
            amount,
            sender_id,
            msg
        );

        assert!(self.field.is_none(), "Game already started");

        if self.first.is_none() {
            log!(
                "first player registered: {} with deposit: {:?}",
                sender_id,
                amount
            );
            self.first = Some(Player::new(sender_id, amount));
            PromiseOrValue::Value(U128::from(0))
        } else if self.second.is_none() {
            assert!(
                self.first.as_ref().unwrap().deposit() == amount,
                "deposit should be: {amount:?}"
            );
            log!(
                "second player registered: {} with deposit: {:?}, game started!",
                sender_id,
                amount
            );
            self.second = Some(Player::new(sender_id, amount));

            log!("create stream for first player");
            let ft_contract_id = env::predecessor_account_id();
            let streaming_id = self.streaming_id.as_ref().unwrap();
            log!("tokens staked: [{}]", ft_contract_id);

            let memo = format!(
                "Roketo transfer: {}",
                self.first.as_ref().unwrap().account()
            );
            let msg = "{\"Create\":{\"request\":{\"balance\":\"200000000000000000000000\",\"owner_id\":\"tic-tac-near.vengone.testnet\",\"receiver_id\":\"vengone.testnet\",\"token_name\":\"wrap.testnet\",\"tokens_per_sec\":\"1000\",\"is_locked\":false,\"is_auto_start_enabled\":false,\"description\":\"{\\\"player\\\":\\\"first\\\"}\"}}}".to_string();
            let promise = wrap::ext(ft_contract_id.clone())
                .with_static_gas(Gas(60 * TGAS))
                .with_attached_deposit(1)
                .ft_transfer_call(streaming_id.clone(), amount, memo, msg);

            let current_account = env::current_account_id();
            let promise = promise.then(
                streaming_roketo::ext(streaming_id.clone()).get_account(current_account.clone()),
            );

            let first_account_id = self.first.as_ref().unwrap().account();
            let promise = promise.then(
                Self::ext(current_account.clone())
                    .query_stream_id_callback(first_account_id.clone()),
            );

            let memo = format!(
                "Roketo transfer: {}",
                self.second.as_ref().unwrap().account()
            );
            let msg = "{\"Create\":{\"request\":{\"balance\":\"200000000000000000000000\",\"owner_id\":\"tic-tac-near.vengone.testnet\",\"receiver_id\":\"vengone1.testnet\",\"token_name\":\"wrap.testnet\",\"tokens_per_sec\":\"1000\",\"is_locked\":false,\"is_auto_start_enabled\":false,\"description\":\"{\\\"player\\\":\\\"second\\\"}\"}}}".to_string();
            let promise = promise.then(
                wrap::ext(ft_contract_id)
                    .with_static_gas(Gas(60 * TGAS))
                    .with_attached_deposit(1)
                    .ft_transfer_call(streaming_id.clone(), amount, memo, msg),
            );

            let promise = promise.then(
                streaming_roketo::ext(streaming_id.clone()).get_account(current_account.clone()),
            );

            let second_account_id = self.second.as_ref().unwrap().account();
            let promise = promise.then(
                Self::ext(current_account).query_stream_id_callback(second_account_id.clone()),
            );
            self.field = Some(Field::new());
            PromiseOrValue::Promise(promise)
        } else {
            panic!("all players are in, registration closed");
        }
    }
}
