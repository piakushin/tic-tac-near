use near_sdk::{ext_contract, json_types::U128, AccountId};

#[ext_contract(wrap)]
trait WrapNear {
    fn ft_transfer_call(receiver_id: AccountId, amount: U128, memo: String, msg: String);

    fn ft_transfer(receiver_id: AccountId, amount: U128);
}
