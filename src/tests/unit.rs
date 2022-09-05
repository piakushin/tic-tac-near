use near_sdk::{test_utils::VMContextBuilder, testing_env, AccountId};

use crate::Contract;

fn to_account_id(s: impl AsRef<str>) -> AccountId {
    match s.as_ref().parse() {
        Ok(account_id) => account_id,
        Err(e) => panic!("Invalid account id: {}, error: {e}", s.as_ref()),
    }
}

fn get_context(predecessor: AccountId) -> VMContextBuilder {
    let mut builder = VMContextBuilder::new();
    builder.predecessor_account_id(predecessor);
    builder
}

#[test]
fn connect_streaming_contract() {
    let context = get_context(to_account_id("alice")).build();
    testing_env!(context);

    let mut contract = Contract {
        field: None,
        turn: None,
        first: None,
        second: None,
        winner: None,
        token_id: None,
        deposit: 0,
        tokens_per_sec: String::new(),
        streaming_id: None,
    };
    let streaming_id = to_account_id("streaming.contract");
    contract.connect_streaming_contract(streaming_id.clone());
    assert_eq!(contract.streaming_id, Some(streaming_id))
}

#[test]
#[should_panic]
fn panic_on_connect_streaming_contract_twice() {
    let context = get_context(to_account_id("alice")).build();
    testing_env!(context);

    let mut contract = Contract {
        field: None,
        turn: None,
        first: None,
        second: None,
        winner: None,
        token_id: None,
        deposit: 0,
        tokens_per_sec: String::new(),
        streaming_id: None,
    };
    let streaming_id = to_account_id("streaming.contract");
    contract.connect_streaming_contract(streaming_id.clone());
    contract.connect_streaming_contract(streaming_id);
}
