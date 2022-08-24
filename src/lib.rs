mod external;
mod field;
mod interface;
mod player;
mod views;

use std::collections::HashMap;

use external::{
    streaming_roketo::streaming_roketo::{self, StreamingRoketoExt},
    wrap_near::wrap,
};
use field::{Field, State};
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    env,
    json_types::U128,
    log, near_bindgen,
    serde_json::Value,
    AccountId, Gas, Promise, PromiseError,
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
    streaming_id: Option<AccountId>,
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
            streaming_id: None,
        }
    }

    pub fn connect_streaming_contract(&mut self, streaming_id: AccountId) {
        assert!(
            self.streaming_id.is_none(),
            "streaming contract is already connected"
        );
        self.streaming_id = Some(streaming_id);
    }

    pub fn start(&mut self) -> Promise {
        let streaming_id = self.streaming_id.as_ref().unwrap();
        let first = self.first.as_ref().unwrap().stream().unwrap().clone();
        let second = self.second.as_ref().unwrap().stream().unwrap();

        self.turn = Some(0);
        let promise = start_stream(streaming_id.clone(), first);

        let promise = promise.then(start_stream(streaming_id.clone(), second.clone()));

        promise.then(pause_stream(streaming_id.clone(), second.clone()))
    }

    pub fn make_turn(&mut self, x: u8, y: u8) -> Promise {
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
            self.check_winner(self.first.as_ref().unwrap(), self.second.as_ref().unwrap())
        } else if rem == 1 {
            log!("first: {}, current: {}", first.account(), current);
            assert!(second.account() == &current, "it's second player's turn");
            field.set_o(x, y);
            *self.turn.as_mut().unwrap() += 1;
            self.check_winner(self.second.as_ref().unwrap(), self.first.as_ref().unwrap())
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
        log!("res: {:?}", res);
        // let withdrawn = res.get("tokens_total_withdrawn").unwrap().as_u64().unwrap() as u128;
        let withdrawn = 0;
        let win_money = if self.first.as_ref().unwrap().account() == &player_id {
            self.first.as_ref().unwrap().deposit().0
        } else if self.second.as_ref().unwrap().account() == &player_id {
            self.second.as_ref().unwrap().deposit().0
        } else {
            unreachable!();
        } - withdrawn;
        log!("reward {} tokens to {}", win_money, player_id);
        wrap::ext(AccountId::new_unchecked("wrap.testnet".to_string()))
            .with_attached_deposit(1)
            .ft_transfer_call(
                player_id,
                U128::from(win_money),
                String::new(),
                String::new(),
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

impl Contract {
    fn check_winner(&self, active: &Player, passive: &Player) -> Promise {
        let field = self.field.as_ref().unwrap();
        let streaming_id = self.streaming_id.as_ref().unwrap();
        match field.get_winner() {
            State::Empty => {
                pause_stream(streaming_id.clone(), active.stream().unwrap().clone()).then(
                    start_stream(streaming_id.clone(), passive.stream().unwrap().clone()),
                )
            }
            State::X => {
                let promise = stop_stream(streaming_id.clone(), active.stream().unwrap().clone());
                let promise = promise.then(stop_stream(
                    streaming_id.clone(),
                    passive.stream().unwrap().clone(),
                ));
                let promise = promise.then(get_stream(
                    streaming_id.clone(),
                    active.stream().unwrap().clone(),
                ));
                let promise = promise.then(
                    Self::ext(AccountId::new_unchecked(String::from(
                        "tic-tac-near.vengone.testnet",
                    )))
                    .query_transferred_tokens_callback(active.account().clone()),
                );
                promise
            }
            State::O => {
                let promise = stop_stream(streaming_id.clone(), active.stream().unwrap().clone());
                let promise = promise.then(stop_stream(
                    streaming_id.clone(),
                    active.stream().unwrap().clone(),
                ));
                let promise = promise.then(get_stream(
                    streaming_id.clone(),
                    active.stream().unwrap().clone(),
                ));
                let promise = promise.then(
                    Self::ext(AccountId::new_unchecked(String::from(
                        "tic-tac-near.vengone.testnet",
                    )))
                    .query_transferred_tokens_callback(passive.account().clone()),
                );
                promise
            }
        }
    }
}

fn streaming(streaming_id: AccountId) -> StreamingRoketoExt {
    streaming_roketo::ext(streaming_id)
        .with_attached_deposit(1)
        .with_static_gas(Gas(60 * TGAS))
}

fn start_stream(streaming_id: AccountId, stream_id: String) -> Promise {
    streaming(streaming_id).start_stream(stream_id)
}

fn pause_stream(streaming_id: AccountId, stream_id: String) -> Promise {
    streaming(streaming_id).pause_stream(stream_id)
}

fn stop_stream(streaming_id: AccountId, stream_id: String) -> Promise {
    streaming(streaming_id).stop_stream(stream_id)
}

fn get_stream(streaming_id: AccountId, stream_id: String) -> Promise {
    streaming(streaming_id).get_stream(stream_id)
}

impl Default for Contract {
    fn default() -> Self {
        Self::new()
    }
}
