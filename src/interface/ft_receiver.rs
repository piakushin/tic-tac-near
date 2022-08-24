use near_contract_standards::fungible_token::receiver::FungibleTokenReceiver;
use near_sdk::{json_types::U128, log, near_bindgen, AccountId, Gas, PromiseOrValue};

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
            "received {:?} tokens from {} with msg: {}",
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
            let promise = wrap::ext(AccountId::new_unchecked("wrap.testnet".to_string()))
                .with_static_gas(Gas(60 * TGAS))
                .with_attached_deposit(1)
                .ft_transfer_call(
                    AccountId::new_unchecked("streaming-roketo.vengone.testnet".to_string()),
                    amount,
                    format!(
                        "Roketo transfer: {}",
                        self.first.as_ref().unwrap().account()
                    ),
                    "{\"Create\":{\"request\":{\"balance\":\"200000000000000000000000\",\"owner_id\":\"tic-tac-near.vengone.testnet\",\"receiver_id\":\"vengone.testnet\",\"token_name\":\"wrap.testnet\",\"tokens_per_sec\":\"1000\",\"is_locked\":false,\"is_auto_start_enabled\":false,\"description\":\"{\\\"player\\\":\\\"first\\\"}\"}}}".to_string(),
                );

            log!("query account status to get stream id");
            let promise = promise.then(
                streaming_roketo::ext(AccountId::new_unchecked(
                    "streaming-roketo.vengone.testnet".to_string(),
                ))
                .get_account(AccountId::new_unchecked(
                    "tic-tac-near.vengone.testnet".to_string(),
                )),
            );

            log!("parse and save stream id");
            let promise = promise.then(
                Self::ext(AccountId::new_unchecked(
                    "tic-tac-near.vengone.testnet".to_string(),
                ))
                .query_stream_id_callback(AccountId::new_unchecked("vengone1.testnet".to_string())),
            );

            let promise = promise.then( wrap::ext(AccountId::new_unchecked("wrap.testnet".to_string()))
                .with_static_gas(Gas(60 * TGAS))
                .with_attached_deposit(1)
                .ft_transfer_call(
                    AccountId::new_unchecked("streaming-roketo.vengone.testnet".to_string()),
                    amount,
                    format!(
                        "Roketo transfer: {}",
                        self.first.as_ref().unwrap().account()
                    ),
                    "{\"Create\":{\"request\":{\"balance\":\"200000000000000000000000\",\"owner_id\":\"tic-tac-near.vengone.testnet\",\"receiver_id\":\"vengone1.testnet\",\"token_name\":\"wrap.testnet\",\"tokens_per_sec\":\"1000\",\"is_locked\":false,\"is_auto_start_enabled\":false,\"description\":\"{\\\"player\\\":\\\"second\\\"}\"}}}".to_string(),
                ));

            log!("query account status to get stream id");
            let promise = promise.then(
                streaming_roketo::ext(AccountId::new_unchecked(
                    "streaming-roketo.vengone.testnet".to_string(),
                ))
                .get_account(AccountId::new_unchecked(
                    "tic-tac-near.vengone.testnet".to_string(),
                )),
            );

            log!("parse and save stream id");
            let promise = promise.then(
                Self::ext(AccountId::new_unchecked(
                    "tic-tac-near.vengone.testnet".to_string(),
                ))
                .query_stream_id_callback(AccountId::new_unchecked("vengone.testnet".to_string())),
            );
            self.field = Some(Field::new());
            PromiseOrValue::Promise(promise)
        } else {
            panic!("all players are in, registration closed");
        }
    }
}
