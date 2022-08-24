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
        if self.turn % 2 == 0 {
            assert!(
                self.first.as_ref().unwrap().account() == &env::current_account_id(),
                "it's first player's turn"
            );
            self.field.as_mut().unwrap().set_x(x, y);
        } else if self.turn % 2 == 1 {
            assert!(
                self.second.as_ref().unwrap().account() == &env::current_account_id(),
                "it's second player's turn"
            );
            self.field.as_mut().unwrap().set_o(x, y);
        } else {
            unreachable!()
        }
        self.turn += 1;
        match self.field.as_ref().unwrap().has_winner() {
            State::Empty => None,
            State::X => Some(self.first.as_ref().unwrap().account().clone()),
            State::O => Some(self.second.as_ref().unwrap().account().clone()),
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
        if self.first.as_ref().unwrap().account() == &player_id {
            self.first.as_mut().unwrap().stream = Some(id.to_string());
        } else if self.second.as_ref().unwrap().account() == &player_id {
            self.second.as_mut().unwrap().stream = Some(id.to_string());
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
