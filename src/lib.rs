mod external;
mod field;
mod interface;
mod player;
mod views;

use std::collections::HashMap;

use external::{streaming_roketo::streaming_roketo, wrap_near::wrap};
use field::{Field, State};
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    env,
    json_types::U128,
    log, near_bindgen,
    serde_json::Value,
    AccountId, Gas, Promise, PromiseError, PromiseOrValue,
};
use player::Player;
use serde::Serialize;

use crate::external::TGAS;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Serialize, Clone)]
pub struct Contract {
    field: Option<Field>,
    turn: Option<u8>,
    first: Option<Player>,
    second: Option<Player>,
}

#[near_bindgen]
impl Contract {
    #[init]
    #[private]
    pub fn new() -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self {
            field: None,
            turn: None,
            first: None,
            second: None,
        }
    }

    pub fn start(&mut self) -> Promise {
        let first = self.first.as_ref().unwrap();
        let second = self.second.as_ref().unwrap();

        self.turn = Some(0);
        let promise = streaming_roketo::ext(AccountId::new_unchecked(
            "streaming-roketo.vengone.testnet".to_string(),
        ))
        .with_attached_deposit(1)
        .with_static_gas(Gas(60 * TGAS))
        .start_stream(first.stream().unwrap().clone());

        let promise = promise.then(
            streaming_roketo::ext(AccountId::new_unchecked(
                "streaming-roketo.vengone.testnet".to_string(),
            ))
            .with_attached_deposit(1)
            .with_static_gas(Gas(60 * TGAS))
            .start_stream(second.stream().unwrap().clone()),
        );

        let promise = promise.then(
            streaming_roketo::ext(AccountId::new_unchecked(
                "streaming-roketo.vengone.testnet".to_string(),
            ))
            .with_attached_deposit(1)
            .with_static_gas(Gas(60 * TGAS))
            .pause_stream(second.stream().unwrap().clone()),
        );
        promise
    }

    pub fn make_turn(&mut self, x: u8, y: u8) -> PromiseOrValue<AccountId> {
        let first = self.first.as_ref().unwrap();
        let second = self.second.as_ref().unwrap();
        let current = env::signer_account_id();
        let field = self.field.as_mut().unwrap();

        let rem = self.turn.as_ref().unwrap() % 2;
        if rem == 0 {
            log!("first: {}, current: {}", first.account(), current);
            assert!(first.account() == &current, "it's first player's turn");
            field.set_x(x, y);
            *self.turn.as_mut().unwrap() += 1;

            match field.has_winner() {
                State::Empty => {
                    let promise = streaming_roketo::ext(AccountId::new_unchecked(
                        "streaming-roketo.vengone.testnet".to_string(),
                    ))
                    .with_attached_deposit(1)
                    .with_static_gas(Gas(60 * TGAS))
                    .pause_stream(first.stream().unwrap().clone())
                    .then(
                        streaming_roketo::ext(AccountId::new_unchecked(
                            "streaming-roketo.vengone.testnet".to_string(),
                        ))
                        .with_attached_deposit(1)
                        .with_static_gas(Gas(60 * TGAS))
                        .start_stream(second.stream().unwrap().clone()),
                    );
                    PromiseOrValue::Promise(promise)
                }
                State::X => {
                    let promise = streaming_roketo::ext(AccountId::new_unchecked(
                        "streaming-roketo.vengone.testnet".to_string(),
                    ))
                    .with_attached_deposit(1)
                    .stop_stream(first.stream().unwrap().clone());
                    streaming_roketo::ext(AccountId::new_unchecked(
                        "streaming-roketo.vengone.testnet".to_string(),
                    ))
                    .with_attached_deposit(1)
                    .stop_stream(second.stream().unwrap().clone());
                    let promise = promise.then({
                        streaming_roketo::ext(AccountId::new_unchecked(
                            "streaming-roketo.vengone.testnet".to_string(),
                        ))
                        .with_attached_deposit(1)
                        .get_stream(first.stream().unwrap().clone())
                        .then(
                            Self::ext(AccountId::new_unchecked(
                                "streaming-roketo.vengone.testnet".to_string(),
                            ))
                            .query_transferred_tokens_callback(first.account().clone()),
                        )
                    });
                    PromiseOrValue::Promise(promise)
                }
                State::O => {
                    let promise = streaming_roketo::ext(AccountId::new_unchecked(
                        "streaming-roketo.vengone.testnet".to_string(),
                    ))
                    .with_attached_deposit(1)
                    .stop_stream(second.stream().unwrap().clone());
                    streaming_roketo::ext(AccountId::new_unchecked(
                        "streaming-roketo.vengone.testnet".to_string(),
                    ))
                    .with_attached_deposit(1)
                    .stop_stream(first.stream().unwrap().clone());
                    let promise = promise.then({
                        streaming_roketo::ext(AccountId::new_unchecked(
                            "streaming-roketo.vengone.testnet".to_string(),
                        ))
                        .with_attached_deposit(1)
                        .get_stream(first.stream().unwrap().clone())
                        .then(
                            Self::ext(AccountId::new_unchecked(
                                "streaming-roketo.vengone.testnet".to_string(),
                            ))
                            .query_transferred_tokens_callback(first.account().clone()),
                        )
                    });
                    PromiseOrValue::Promise(promise)
                }
            }
        } else if rem == 1 {
            assert!(second.account() == &current, "it's second player's turn");
            field.set_o(x, y);
            *self.turn.as_mut().unwrap() += 1;

            match field.has_winner() {
                State::Empty => {
                    let promise = streaming_roketo::ext(AccountId::new_unchecked(
                        "streaming-roketo.vengone.testnet".to_string(),
                    ))
                    .with_attached_deposit(1)
                    .with_static_gas(Gas(60 * TGAS))
                    .pause_stream(second.stream().unwrap().clone())
                    .then(
                        streaming_roketo::ext(AccountId::new_unchecked(
                            "streaming-roketo.vengone.testnet".to_string(),
                        ))
                        .with_attached_deposit(1)
                        .with_static_gas(Gas(60 * TGAS))
                        .start_stream(first.stream().unwrap().clone()),
                    );
                    PromiseOrValue::Promise(promise)
                }
                State::X => {
                    let promise = streaming_roketo::ext(AccountId::new_unchecked(
                        "streaming-roketo.vengone.testnet".to_string(),
                    ))
                    .stop_stream(first.stream().unwrap().clone());
                    streaming_roketo::ext(AccountId::new_unchecked(
                        "streaming-roketo.vengone.testnet".to_string(),
                    ))
                    .stop_stream(second.stream().unwrap().clone());
                    let promise = promise.then({
                        streaming_roketo::ext(AccountId::new_unchecked(
                            "streaming-roketo.vengone.testnet".to_string(),
                        ))
                        .with_attached_deposit(1)
                        .get_stream(first.stream().unwrap().clone())
                        .then(
                            Self::ext(AccountId::new_unchecked(
                                "streaming-roketo.vengone.testnet".to_string(),
                            ))
                            .query_transferred_tokens_callback(first.account().clone()),
                        )
                    });
                    PromiseOrValue::Promise(promise)
                }
                State::O => {
                    let promise = streaming_roketo::ext(AccountId::new_unchecked(
                        "streaming-roketo.vengone.testnet".to_string(),
                    ))
                    .stop_stream(second.stream().unwrap().clone());
                    streaming_roketo::ext(AccountId::new_unchecked(
                        "streaming-roketo.vengone.testnet".to_string(),
                    ))
                    .stop_stream(first.stream().unwrap().clone());
                    let promise = promise.then({
                        streaming_roketo::ext(AccountId::new_unchecked(
                            "streaming-roketo.vengone.testnet".to_string(),
                        ))
                        .with_attached_deposit(1)
                        .get_stream(first.stream().unwrap().clone())
                        .then(
                            Self::ext(AccountId::new_unchecked(
                                "streaming-roketo.vengone.testnet".to_string(),
                            ))
                            .query_transferred_tokens_callback(first.account().clone()),
                        )
                    });
                    PromiseOrValue::Promise(promise)
                }
            }
        } else {
            unreachable!()
        }
    }

    #[private]
    pub fn query_transferred_tokens_callback(
        &mut self,
        #[callback_result] call_result: Result<HashMap<String, Value>, PromiseError>,
        player_id: AccountId,
    ) -> Promise {
        let res = call_result.unwrap();
        let withdrawn = res.get("tokens_total_withdrawn").unwrap().as_u64().unwrap() as u128;
        let win_money = self.first.as_ref().unwrap().deposit().0 - withdrawn;
        wrap::ext(AccountId::new_unchecked("wrap.testnet".to_string())).ft_transfer(
            self.first.as_ref().unwrap().account().clone(),
            U128::from(win_money),
        )
    }

    #[private]
    pub fn query_stream_id_callback(
        &mut self,
        #[callback_result] call_result: Result<HashMap<String, Value>, PromiseError>,
        player_id: AccountId,
    ) {
        let res = call_result.unwrap();
        let id = res.get("last_created_stream").unwrap().as_str().unwrap();
        log!("[{}] stream id: {}", player_id, id);

        let first = self.first.as_mut().unwrap();
        let second = self.second.as_mut().unwrap();
        if first.account() == &player_id {
            first.stream = Some(id.to_string());
        } else if second.account() == &player_id {
            second.stream = Some(id.to_string());
        } else {
            panic!("unknown player ID");
        }
    }
}

impl Default for Contract {
    fn default() -> Self {
        Self::new()
    }
}
