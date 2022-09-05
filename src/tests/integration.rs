use anyhow::Result;
use near_sdk::serde_json::json;
use tokio::fs::read;
use workspaces::{sandbox, testnet};

const WASM_PATH: &str = "./out/tic_tac_near.wasm";

const CONNECT_STREAMING_CONTRACT: &str = "connect_streaming_contract";

#[tokio::test]
async fn test_contract() -> Result<()> {
    let sandbox_worker = sandbox().await?;
    let testnet_worker = testnet().await?;

    let wasm = read(WASM_PATH).await?;

    let game_contract = sandbox_worker.dev_deploy(&wasm).await?;

    let args = json!(
        {
            "streaming_id": "streaming-roketo.vengone.testnet"
        }
    );
    let outcome = game_contract
        .call(CONNECT_STREAMING_CONTRACT)
        .args_json(args)
        .transact()
        .await?;
    println!("{CONNECT_STREAMING_CONTRACT} outcome: {outcome:#?}");

    let first_player = sandbox_worker
        .root_account()?
        .create_subaccount("first")
        .initial_balance(10u128.pow(25))
        .transact()
        .await?
        .into_result()?;
    println!("first player: {first_player:?}\n");

    let wrap_near_contract = sandbox_worker
        .import_contract(&"wrap.testnet".parse().unwrap(), &testnet_worker)
        .transact()
        .await?;
    println!("wrap.testnet contract imported to sandbox\n");

    let outcome = wrap_near_contract.call("new").transact().await?;
    println!("wrap - new - outcome: {outcome:#?}");

    let args = json!(
        {
            "account_id": game_contract.as_account().id(),
        }
    );
    let outcome = wrap_near_contract
        .call("storage_deposit")
        .args_json(args)
        .deposit(12_500_000_000_000_000_000_000)
        .transact()
        .await?;

    println!("storage deposit outcome: {outcome:#?}");
    todo!()
}
