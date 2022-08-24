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

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    field: Field,
    turn: u8,
    first: Player,
    second: Option<Player>,
}

#[near_bindgen]
impl Contract {
    #[init]
    #[private]
    pub fn new() -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self {
            field: Field::new(),
            turn: 0,
            first: Player::new(env::current_account_id()),
            second: None,
        }
    }

    pub fn add_second_player(&mut self, second_player: AccountId) -> AccountId {
        assert!(self.second.is_none(), "second player already registered");
        self.second = Some(Player::new(second_player.clone()));
        second_player
    }

    pub fn make_turn(&mut self, x: u8, y: u8) -> Option<AccountId> {
        if self.turn % 2 == 0 {
            assert!(
                self.first.account() == &env::current_account_id(),
                "it's first player's turn"
            );
            self.field.set_x(x, y);
        } else if self.turn % 2 == 1 {
            assert!(
                self.second.as_ref().unwrap().account() == &env::current_account_id(),
                "it's second player's turn"
            );
            self.field.set_o(x, y);
        } else {
            unreachable!()
        }
        self.turn += 1;
        match self.field.has_winner() {
            State::Empty => None,
            State::X => Some(self.first.account().clone()),
            State::O => Some(self.second.as_ref().unwrap().account().clone()),
        }
    }
}

impl Default for Contract {
    fn default() -> Self {
        Self::new()
    }
}
