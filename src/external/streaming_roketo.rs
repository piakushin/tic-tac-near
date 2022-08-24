use near_sdk::{ext_contract, AccountId};

#[ext_contract(streaming_roketo)]
trait StreamingRoketo {
    fn get_account(account_id: AccountId) -> String;
}
