use anyhow::Result;
use tokio::fs::read;
use workspaces::sandbox;

const WASM_PATH: &str = "./out/tic_tac_near.wasm";

#[tokio::test]
async fn test_contract() -> Result<()> {
    let worker = sandbox().await?;
    let wasm = read(WASM_PATH).await?;

    let contract = worker.dev_deploy(&wasm).await?;
    todo!()
}
