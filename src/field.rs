use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use serde::Serialize;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Field {
    inner: [[State; 3]; 3],
}

impl Field {
    pub fn new() -> Self {
        Self {
            inner: [[State::Empty; 3]; 3],
        }
    }

    pub fn set_x(&mut self, x: u8, y: u8) {
        self.set(x, y, State::X)
    }

    pub fn set_o(&mut self, x: u8, y: u8) {
        self.set(x, y, State::O)
    }

    fn set(&mut self, x: u8, y: u8, value: State) {
        let state = self
            .inner
            .get_mut(y as usize)
            .unwrap()
            .get_mut(x as usize)
            .unwrap();
        if state.is_empty() {
            *state = value;
        } else {
            panic!("occupied");
        }
    }

    pub fn get_winner(&self) -> State {
        if self.inner[0][0] == self.inner[0][1] && self.inner[0][0] == self.inner[0][2] {
            self.inner[0][0]
        } else if self.inner[1][0] == self.inner[1][1] && self.inner[1][0] == self.inner[1][2] {
            self.inner[1][0]
        } else if self.inner[2][0] == self.inner[2][1] && self.inner[2][0] == self.inner[2][2] {
            self.inner[2][0]
        } else if self.inner[0][0] == self.inner[1][0] && self.inner[2][0] == self.inner[0][0] {
            self.inner[0][0]
        } else if self.inner[0][1] == self.inner[1][1] && self.inner[2][1] == self.inner[0][1] {
            self.inner[0][1]
        } else if self.inner[0][2] == self.inner[1][2] && self.inner[2][2] == self.inner[0][2] {
            self.inner[0][2]
        } else if self.inner[0][0] == self.inner[1][1] && self.inner[2][2] == self.inner[0][0] {
            self.inner[0][0]
        } else if self.inner[2][0] == self.inner[1][1] && self.inner[0][2] == self.inner[2][0] {
            self.inner[2][0]
        } else {
            State::Empty
        }
    }
}

impl Default for Field {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Clone, Copy, PartialEq, Eq)]
#[serde(crate = "near_sdk::serde")]
pub enum State {
    Empty,
    X,
    O,
}

impl State {
    fn is_empty(self) -> bool {
        matches!(self, Self::Empty)
    }
}

impl Default for State {
    fn default() -> Self {
        Self::Empty
    }
}
