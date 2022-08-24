use near_sdk::near_bindgen;

use crate::{field::Field, Contract, ContractExt};

#[near_bindgen]
impl Contract {
    pub fn get_field(&self) -> Field {
        self.field.clone()
    }
}
