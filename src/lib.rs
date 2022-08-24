mod external;
mod field;
mod interface;
mod player;
mod views;

use std::collections::HashMap;

use field::{Field, State};
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    env, log, near_bindgen,
    serde_json::Value,
    AccountId, PromiseError,
};
use player::Player;
use serde::Serialize;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Serialize, Clone)]
pub struct Contract {
    field: Option<Field>,
    turn: u8,
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
            turn: 0,
            first: None,
            second: None,
        }
    }

    pub fn make_turn(&mut self, x: u8, y: u8) -> Option<AccountId> {
        let first = self.first.as_ref().unwrap().account();
        let second = self.second.as_ref().unwrap().account();
        let current = env::current_account_id();
        let field = self.field.as_mut().unwrap();

        let rem = self.turn % 2;
        if rem == 0 {
            assert!(first == &current, "it's first player's turn");
            field.set_x(x, y);
        } else if rem == 1 {
            assert!(second == &current, "it's second player's turn");
            field.set_o(x, y);
        } else {
            unreachable!()
        }
        self.turn += 1;
        match field.has_winner() {
            State::Empty => None,
            State::X => Some(first.clone()),
            State::O => Some(second.clone()),
        }
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
