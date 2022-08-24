mod field;
mod interface;
mod player;
mod views;

use field::{Field, State};
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    env, near_bindgen, AccountId,
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
}

impl Default for Contract {
    fn default() -> Self {
        Self::new()
    }
}
