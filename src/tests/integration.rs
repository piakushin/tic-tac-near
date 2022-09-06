use std::str::FromStr;

use anyhow::Result;
use near_sdk::serde_json::{self, json, Value};
use tokio::fs::read;
use workspaces::{
    network::{Sandbox, Testnet},
    sandbox, testnet, Account, AccountId, Contract, Worker,
};

const WASM_PATH: &str = "./out/tic_tac_near.wasm";

const CONNECT_STREAMING_CONTRACT: &str = "connect_streaming_contract";
const STORAGE_DEPOSIT_CALL: &str = "storage_deposit";
const FT_TRANSFER_CALL: &str = "ft_transfer_call";

const WRAP_CONTRACT_ID: &str = "wrap.testnet";
const STREAMING_CONTRACT_ID: &str = "streaming-roketo.vengone.testnet";

const INITIAL_BALANCE: u128 = 10u128.pow(25);
const STORAGE_DEPOSIT: u128 = 12_500_000_000_000_000_000_000;

#[tokio::test]
async fn test_contract() -> Result<()> {
    let sandbox = sandbox().await?;
    let testnet = testnet().await?;

    let game_contract = deploy(&sandbox).await?;
    let wrap_contract = prepare_wrap_contract(
        &sandbox,
        &testnet,
        WRAP_CONTRACT_ID,
        game_contract.as_account(),
    )
    .await?;

    let streaming_contract = import_contract(&sandbox, &testnet, STREAMING_CONTRACT_ID).await?;
    wrap_register_account(streaming_contract.as_account(), wrap_contract.id()).await?;
    connect_streaming_contract(&game_contract, streaming_contract.id()).await?;

    let first = create_account(&sandbox, "first").await?;
    let second = create_account(&sandbox, "second").await?;

    wrap_register_account(&first, wrap_contract.id()).await?;
    wrap_register_account(&second, wrap_contract.id()).await?;

    place_bet(
        &first,
        wrap_contract.id(),
        game_contract.id(),
        10u128.pow(18),
    )
    .await?;
    place_bet(
        &second,
        wrap_contract.id(),
        game_contract.id(),
        10u128.pow(18),
    )
    .await?;

    let outcome = first.call(game_contract.id(), "start").transact().await?;
    println!("{} - start - {outcome:#?}", game_contract.id());

    let outcome = first
        .call(game_contract.id(), "make_turn")
        .args_json(move_args(1, 1))
        .transact()
        .await?;
    println!("{} - make_turn - {outcome:#?}", game_contract.id());

    let outcome = second
        .call(game_contract.id(), "make_turn")
        .args_json(move_args(0, 0))
        .transact()
        .await?;

    println!("{} - make_turn - {outcome:#?}", game_contract.id());

    let outcome = first
        .call(game_contract.id(), "make_turn")
        .args_json(move_args(1, 0))
        .transact()
        .await?;

    println!("{} - make_turn - {outcome:#?}", game_contract.id());

    let outcome = second
        .call(game_contract.id(), "make_turn")
        .args_json(move_args(0, 1))
        .transact()
        .await?;

    println!("{} - make_turn - {outcome:#?}", game_contract.id());

    let outcome = first
        .call(game_contract.id(), "make_turn")
        .args_json(move_args(1, 2))
        .transact()
        .await?;

    println!("{} - make_turn - {outcome:#?}", game_contract.id());

    let outcome = first
        .call(game_contract.id(), "claim_reward")
        .transact()
        .await?;

    println!("{} - claim_reward - {outcome:#?}", game_contract.id());

    let args = json!(
        {
            "account_id": first.id(),
        }
    );
    let args = serde_json::to_vec(&args)?;
    let outcome = wrap_contract.view("ft_balance_of", args).await?;

    println!("{} - ft_balance_of - {outcome:#?}", wrap_contract.id());

    Ok(())
}

async fn deploy(worker: &Worker<Sandbox>) -> Result<Contract> {
    let wasm = read(WASM_PATH).await?;

    let contract = worker.dev_deploy(&wasm).await?;

    Ok(contract)
}

async fn import_contract(
    sandbox: &Worker<Sandbox>,
    testnet: &Worker<Testnet>,
    contract_id: &str,
) -> Result<Contract> {
    let contract_id = AccountId::from_str(contract_id)?;
    let contract = sandbox
        .import_contract(&contract_id, testnet)
        .transact()
        .await?;
    println!("{:?} contract imported to sandbox\n", contract.id());

    Ok(contract)
}

async fn prepare_wrap_contract(
    sandbox: &Worker<Sandbox>,
    testnet: &Worker<Testnet>,
    contract_id: &str,
    game_account: &Account,
) -> Result<Contract> {
    let contract = import_contract(sandbox, testnet, WRAP_CONTRACT_ID).await?;
    let outcome = contract.call("new").transact().await?;
    println!(
        "{contract_id} - new - outcome: {}\n",
        if outcome.outcome().is_success() {
            "OK"
        } else {
            "FAILED"
        }
    );

    wrap_register_account(game_account, contract.id()).await?;

    Ok(contract)
}

async fn wrap_register_account(account: &Account, contract_id: &AccountId) -> Result<()> {
    let args = json!(
        {
            "account_id": account.id(),
        }
    );
    let outcome = account
        .call(contract_id, STORAGE_DEPOSIT_CALL)
        .args_json(args)
        .deposit(STORAGE_DEPOSIT)
        .transact()
        .await?;

    println!(
        "{contract_id} - {STORAGE_DEPOSIT_CALL} - outcome: {}\n",
        if outcome.outcome().is_success() {
            "OK"
        } else {
            "FAILED"
        }
    );

    let outcome = account
        .call(contract_id, "near_deposit")
        .deposit(3 * 10u128.pow(23))
        .transact()
        .await?;

    println!(
        "{contract_id} - near_deposit - outcome: {}\n",
        if outcome.outcome().is_success() {
            "OK"
        } else {
            "FAILED"
        }
    );
    Ok(())
}

async fn create_account(sandbox: &Worker<Sandbox>, id: &str) -> Result<Account> {
    let account = sandbox
        .root_account()?
        .create_subaccount(id)
        .initial_balance(INITIAL_BALANCE)
        .transact()
        .await?
        .into_result()?;
    println!("{id} created: {account:?}\n");
    Ok(account)
}

async fn connect_streaming_contract(game: &Contract, streaming_id: &str) -> Result<()> {
    let args = json!({ "streaming_id": streaming_id });
    let outcome = game
        .call(CONNECT_STREAMING_CONTRACT)
        .args_json(args)
        .transact()
        .await?;
    println!(
        "{:?} - {CONNECT_STREAMING_CONTRACT} outcome: {}",
        game.id(),
        if outcome.outcome().is_success() {
            "OK"
        } else {
            "FAILED"
        }
    );
    Ok(())
}

async fn place_bet(
    player: &Account,
    wrap_id: &AccountId,
    game_id: &str,
    amount: u128,
) -> Result<()> {
    let args = json!(
        {
            "receiver_id": game_id,
            "amount": amount.to_string(),
            "msg": "{\"tokens_per_sec\": \"10000\"}"
        }
    );
    let outcome = player
        .call(wrap_id, FT_TRANSFER_CALL)
        .args_json(args)
        .deposit(1)
        .max_gas()
        .transact()
        .await?;

    println!("{wrap_id} - {FT_TRANSFER_CALL} - outcome: {outcome:#?}",);
    Ok(())
}

fn move_args(x: u8, y: u8) -> Value {
    json!(
        {
            "x": x,
            "y": y,
        }
    )
}
