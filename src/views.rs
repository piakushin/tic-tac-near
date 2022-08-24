use near_sdk::{json_types::U128, near_bindgen, AccountId};
use serde::Serialize;

use crate::{field::Field, Contract, ContractExt};

#[near_bindgen]
impl Contract {
    pub fn status(&self) -> Status {
        Status::new(
            self.first.as_ref().map(|player| player.account().clone()),
            self.first.as_ref().map(|player| player.deposit()),
            self.first
                .as_ref()
                .and_then(|player| player.stream().cloned()),
            self.second.as_ref().map(|player| player.account().clone()),
            self.second.as_ref().map(|player| player.deposit()),
            self.second
                .as_ref()
                .and_then(|player| player.stream().cloned()),
        )
    }

    pub fn get_field(&self) -> Option<Field> {
        self.field.clone()
    }
}

#[derive(Serialize)]
pub struct Status {
    first_player_id: Option<AccountId>,
    first_player_deposit: Option<U128>,
    first_player_stream: Option<String>,

    second_player_id: Option<AccountId>,
    second_player_deposit: Option<U128>,
    second_player_stream: Option<String>,
}

impl Status {
    pub fn new(
        first_player_id: Option<AccountId>,
        first_player_deposit: Option<U128>,
        first_player_stream: Option<String>,
        second_player_id: Option<AccountId>,
        second_player_deposit: Option<U128>,
        second_player_stream: Option<String>,
    ) -> Self {
        Self {
            first_player_id,
            first_player_deposit,
            first_player_stream,
            second_player_id,
            second_player_deposit,
            second_player_stream,
        }
    }
}
