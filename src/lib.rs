mod external;
mod field;
mod interface;
mod player;
mod views;

use std::collections::HashMap;

use external::{
    streaming_roketo::streaming_roketo::{self, StreamingRoketoExt},
    token::token,
};
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
    winner: Option<AccountId>,
    token_id: Option<AccountId>,
    deposit: u128,
    tokens_per_sec: String,
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
            winner: None,
            deposit: 0,
            streaming_id: None,
            token_id: None,
            tokens_per_sec: String::new(),
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
        let first_player_stream = self
            .first_player()
            .stream()
            .expect("first players stream was not registered")
            .clone();
        let second_player_stream = self
            .second_player()
            .stream()
            .expect("second players stream was not registered")
            .clone();

        self.turn = Some(0);
        let streaming_id = self.streaming_id().clone();

        log!(
            "first stream: {}, second: {}",
            first_player_stream,
            second_player_stream
        );
        let start_first_player_stream =
            start_stream(streaming_id.clone(), first_player_stream.clone(), None);
        let pause_first_player_stream =
            pause_stream(streaming_id.clone(), first_player_stream, None);
        let start_second_player_stream = start_stream(streaming_id, second_player_stream, None);

        start_first_player_stream
            .then(pause_first_player_stream)
            .then(start_second_player_stream)
    }

    pub fn make_turn(&mut self, x: u8, y: u8) -> Promise {
        let current = env::signer_account_id();

        let rem = self
            .turn
            .as_ref()
            .expect("method start wasn't called: turns is not initialized")
            % 2;
        if rem == 0 {
            assert!(
                self.first_player().account() == &current,
                "it's first player's turn"
            );
            self.field
                .as_mut()
                .expect("method start wasn't called: field is not initialized")
                .set_x(x, y);
            *self
                .turn
                .as_mut()
                .expect("method start wasn't called: turns is not initialized") += 1;
            self.check_winner(self.first_player(), self.second_player())
        } else if rem == 1 {
            assert!(
                self.second_player().account() == &current,
                "it's second player's turn"
            );
            self.field
                .as_mut()
                .expect("method start wasn't called: field is not initialized")
                .set_o(x, y);
            *self
                .turn
                .as_mut()
                .expect("method start wasn't called: turns is not initialized") += 1;
            self.check_winner(self.second_player(), self.first_player())
        } else {
            unreachable!()
        }
    }

    pub fn reset(&mut self) {
        *self = Self {
            field: None,
            turn: None,
            first: None,
            second: None,
            winner: None,
            deposit: 0,
            token_id: None,
            streaming_id: None,
            tokens_per_sec: String::new(),
        };
    }

    pub fn claim_reward(&mut self) -> Promise {
        let streaming_id = self.streaming_id();
        let signer = env::signer_account_id();
        let current_id = env::current_account_id();

        assert!(
            &signer == self.winner.as_ref().expect("there is no reward to claim"),
            "wrong winner"
        );

        let (winner, loser) = if &signer == self.first_player().account() {
            (self.first_player(), self.second_player())
        } else if &signer == self.second_player().account() {
            (self.second_player(), self.first_player())
        } else {
            unreachable!()
        };

        let get_winner_stream_details =
            get_stream(streaming_id.clone(), winner.stream().unwrap().clone(), None);
        let query_winner_transferred_tokens = Self::ext(current_id.clone())
            .query_transferred_tokens_callback(winner.account().clone());
        let get_loser_stream_details =
            get_stream(streaming_id.clone(), loser.stream().unwrap().clone(), None);
        let query_loser_transferred_tokens =
            Self::ext(current_id).query_transferred_tokens_callback(loser.account().clone());

        get_winner_stream_details
            .then(query_winner_transferred_tokens)
            .then(get_loser_stream_details)
            .then(query_loser_transferred_tokens)
    }

    #[private]
    pub fn query_transferred_tokens_callback(
        &mut self,
        #[callback_result] call_result: Result<HashMap<String, Value>, PromiseError>,
        player_id: AccountId,
    ) -> Promise {
        let res = call_result.unwrap();
        let withdrawn: u128 = res
            .get("tokens_total_withdrawn")
            .and_then(|v| v.as_str())
            .expect("unexpected response from roke.to contract")
            .parse()
            .expect("couldn't parse tokens amount in roke.to response");

        log!("withdrawn {} to {}", withdrawn, player_id);

        let win_money = if self.first_player().account() == &player_id {
            self.first_player().deposit().0
        } else if self.second_player().account() == &player_id {
            self.second_player().deposit().0
        } else {
            unreachable!();
        } - withdrawn;
        let win_money = win_money / 100 * 90;
        log!("reward {} tokens (90%) to {}", win_money, player_id);
        token::ext(self.token_id.as_ref().unwrap().clone())
            .with_attached_deposit(1)
            .ft_transfer(player_id, U128::from(win_money), String::new())
    }

    #[private]
    pub fn query_stream_id_callback(
        &mut self,
        #[callback_result] call_result: Result<HashMap<String, Value>, PromiseError>,
        player_id: AccountId,
    ) -> U128 {
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
        U128(0)
    }

    #[private]
    pub fn set_winner(&mut self, winner: AccountId) {
        self.winner = Some(winner);
    }
}

impl Contract {
    fn streaming_id(&self) -> &AccountId {
        self.streaming_id
            .as_ref()
            .expect("streaming id should be connected by now")
    }

    fn first_player(&self) -> &Player {
        self.first.as_ref().expect("first player is not registered")
    }

    fn second_player(&self) -> &Player {
        self.second
            .as_ref()
            .expect("second player is not registered")
    }

    fn register_first_player(
        &mut self,
        account: AccountId,
        token_id: AccountId,
        deposit: U128,
        tokens_per_sec: String,
    ) -> PromiseOrValue<U128> {
        assert!(self.token_id.is_none(), "somehow token ID is already set");

        log!("game token set to: {}", token_id);
        self.token_id = Some(token_id);

        log!("deposit set to: {}", deposit.0);
        self.deposit = deposit.0;

        log!("tokens streaming rate set to: {}/sec", tokens_per_sec);
        self.tokens_per_sec = tokens_per_sec;

        log!("first player registered: {} ", account,);
        self.first = Some(Player::new(account, deposit));
        PromiseOrValue::Value(U128::from(0))
    }

    fn check_winner(&self, active: &Player, passive: &Player) -> Promise {
        match self.field.as_ref().unwrap().get_winner() {
            State::Empty => pause_stream(
                self.streaming_id().clone(),
                passive.stream().unwrap().clone(),
                None,
            )
            .then(start_stream(
                self.streaming_id().clone(),
                active.stream().unwrap().clone(),
                None,
            )),
            _ => {
                log!("player {} WON!", active.account());
                let stop_winner_stream = stop_stream(
                    self.streaming_id().clone(),
                    active.stream().unwrap().clone(),
                    Some(Gas(100 * TGAS)),
                );
                let stop_loser_stream = stop_stream(
                    self.streaming_id().clone(),
                    passive.stream().unwrap().clone(),
                    Some(Gas(100 * TGAS)),
                );
                let set_winner =
                    Self::ext(env::current_account_id()).set_winner(active.account().clone());
                stop_winner_stream.then(stop_loser_stream).then(set_winner)
            }
        }
    }
}

fn streaming(streaming_id: AccountId, gas: Option<Gas>) -> StreamingRoketoExt {
    let streaming_ext = streaming_roketo::ext(streaming_id).with_attached_deposit(1);
    if let Some(gas) = gas {
        streaming_ext.with_static_gas(gas)
    } else {
        streaming_ext
    }
}

fn start_stream(streaming_id: AccountId, stream_id: String, gas: Option<Gas>) -> Promise {
    streaming(streaming_id, gas).start_stream(stream_id)
}

fn pause_stream(streaming_id: AccountId, stream_id: String, gas: Option<Gas>) -> Promise {
    streaming(streaming_id, gas).pause_stream(stream_id)
}

fn stop_stream(streaming_id: AccountId, stream_id: String, gas: Option<Gas>) -> Promise {
    streaming(streaming_id, gas).stop_stream(stream_id)
}

fn get_stream(streaming_id: AccountId, stream_id: String, gas: Option<Gas>) -> Promise {
    streaming(streaming_id, gas).get_stream(stream_id)
}

impl Default for Contract {
    fn default() -> Self {
        Self::new()
    }
}
