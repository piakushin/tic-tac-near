use near_sdk::near_bindgen;
use serde::Serialize;

use crate::{field::Field, player::Player, Contract, ContractExt};

#[near_bindgen]
impl Contract {
    pub fn status(&self) -> Status {
        Status {
            first_player: self.first.clone(),
            second_player: self.second.clone(),
        }
    }

    pub fn get_field(&self) -> Option<Field> {
        self.field.clone()
    }
}

#[derive(Serialize)]
pub struct Status {
    first_player: Option<Player>,
    second_player: Option<Player>,
}
