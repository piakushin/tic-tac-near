use near_sdk::AccountId;
use serde::Serialize;

mod ft_receiver;

#[derive(Serialize)]
struct RoketoStreamingCreateRequest {
    balance: String,
    owner_id: AccountId,
    receiver_id: AccountId,
    token_name: AccountId,
    tokens_per_sec: String,
    is_locked: bool,
    is_auto_start_enabled: bool,
    description: String,
}
